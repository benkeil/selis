//! Turns a resolved [`Table`] model + [`crate::layout`] geometry into the
//! final ANSI string.

use std::collections::HashMap;

use unicode_width::UnicodeWidthStr;

use crate::border::{BorderChars, Borders};
use crate::layout::border_grid::BorderGrid;
use crate::layout::grid::{self, Grid, GridSlot};
use crate::layout::width::{self, SectionGrid};
use crate::model::column::Column;
use crate::model::section::Section;
use crate::model::table::Table;
use crate::style::{Align, Style};
use crate::text::truncate_with_ellipsis;

/// Metadata about one section as laid out within the overall table (its
/// resolved grid, and the global row index its first row starts at).
struct SectionInfo<'a> {
    section: &'a Section,
    grid: Grid,
    row_offset: usize,
}

/// A fully-resolved origin cell, ready to be rendered. Its `content` starts
/// out only case-transformed; [`truncate_resolved_cells`] rewrites it to fit
/// the table's final, resolved column widths once those are known.
struct ResolvedCell {
    colspan: usize,
    content: String,
    style: Style,
}

/// A column's resolved `min_width`/`width`/`max_width`, after cascading
/// table-wide and section-scoped [`Column`] overrides together.
#[derive(Default)]
struct ColumnConstraints {
    min_width: Option<usize>,
    width: Option<usize>,
    max_width: Option<usize>,
}

/// Renders `table` to a single ANSI string (no trailing newline).
pub fn render(table: &Table) -> String {
    let mut out = String::new();
    if let Some(caption) = &table.caption_top {
        out.push_str(caption);
        out.push('\n');
    }

    let column_count = table.column_count().unwrap_or(0);
    if column_count > 0 {
        out.push_str(&render_grid(table, column_count));
        out.push('\n');
    }

    if let Some(caption) = &table.caption_bottom {
        out.push_str(caption);
    } else if out.ends_with('\n') {
        out.pop();
    }

    out
}

fn render_grid(table: &Table, column_count: usize) -> String {
    let mut infos = Vec::new();
    let mut row_offset = 0;
    if let Some(header) = &table.header
        && table.show_header
    {
        infos.push(SectionInfo {
            section: header,
            grid: grid::resolve(header, column_count),
            row_offset,
        });
        row_offset += header.rows.len();
    }
    infos.push(SectionInfo {
        section: &table.body,
        grid: grid::resolve(&table.body, column_count),
        row_offset,
    });
    row_offset += table.body.rows.len();
    if let Some(footer) = &table.footer
        && table.show_footer
    {
        infos.push(SectionInfo {
            section: footer,
            grid: grid::resolve(footer, column_count),
            row_offset,
        });
        row_offset += footer.rows.len();
    }
    let total_rows = row_offset;

    let section_grids: Vec<SectionGrid<'_>> = infos
        .iter()
        .map(|info| SectionGrid { section: info.section, grid: &info.grid, row_offset: info.row_offset })
        .collect();

    // Resolve style/borders/content for every origin cell, keyed by its
    // global (row, col). Content is only case-transformed here, not yet
    // truncated — truncation happens in a later pass, once the table's
    // final column widths (natural width, narrowed by any `min_width`/
    // `width`/`max_width`/`cap_to_width`) are known.
    let mut resolved_cells: HashMap<(usize, usize), ResolvedCell> = HashMap::new();
    let mut resolved_borders: HashMap<(usize, usize), Borders> = HashMap::new();

    for info in &infos {
        for (local_row, row) in info.grid.slots.iter().enumerate() {
            for (col, slot) in row.iter().enumerate() {
                if let GridSlot::Origin { source_row, source_cell, colspan, .. } = slot {
                    let global_row = info.row_offset + local_row;
                    let cell = source_cell.map(|idx| &info.section.rows[*source_row].cells[idx]);
                    let column_override = info.section.columns.get(&col);
                    let table_column_override = table.columns.get(&col);

                    let style = Style::cascade([
                        Some(&table.style),
                        table_column_override.map(|c| &c.style),
                        Some(&info.section.style),
                        column_override.map(|c| &c.style),
                        Some(&info.section.rows[*source_row].style),
                        cell.map(|c| &c.style),
                    ]);
                    let borders = [
                        table.borders,
                        table_column_override.and_then(|c| c.borders),
                        info.section.borders,
                        column_override.and_then(|c| c.borders),
                        info.section.rows[*source_row].borders,
                        cell.and_then(|c| c.borders),
                    ]
                    .into_iter()
                    .flatten()
                    .last()
                    .unwrap_or(Borders::ALL);

                    let content = cell.map(|c| c.content.clone()).unwrap_or_default();
                    let content = match cell.and_then(|c| c.truncate_to) {
                        Some(max_width) => truncate_with_ellipsis(&content, max_width, &table.ellipsis),
                        None => content,
                    };
                    let content = match style.resolved_case() {
                        Some(case) => case.apply(&content),
                        None => content,
                    };
                    resolved_cells.insert(
                        (global_row, col),
                        ResolvedCell { colspan: *colspan, content, style },
                    );
                    resolved_borders.insert((global_row, col), borders);
                }
            }
        }
    }

    // Natural column widths, from the (untruncated, case-transformed)
    // resolved content.
    let content_overrides: HashMap<(usize, usize), usize> =
        resolved_cells.iter().map(|(&pos, rc)| (pos, rc.content.width())).collect();
    let natural_widths =
        width::compute_column_widths(&section_grids, column_count, 1, &content_overrides);

    let constraints = resolve_column_constraints(table, &infos, column_count);
    let mut widths = apply_column_constraints(natural_widths, &constraints);
    if let Some(target) = table.cap_width {
        if let Some(flex_col) = resolve_flex_column(table, &infos, column_count) {
            apply_flex_width(&mut widths, &constraints, flex_col, target, column_count);
        }
        shrink_to_fit(&mut widths, &constraints, target);
    }
    truncate_resolved_cells(&mut resolved_cells, &widths, column_count, &table.ellipsis);

    // Build the block-id / borders matrices the BorderGrid needs, by
    // propagating each slot's origin's resolved values to every position it
    // covers (including spanned positions).
    let mut block_id = vec![vec![(0usize, 0usize); column_count]; total_rows];
    let mut borders_matrix = vec![vec![Borders::ALL; column_count]; total_rows];
    for info in &infos {
        for (local_row, row) in info.grid.slots.iter().enumerate() {
            let global_row = info.row_offset + local_row;
            for (col, slot) in row.iter().enumerate() {
                let (origin_local_row, origin_col) = slot.origin(local_row, col);
                let origin_global_row = info.row_offset + origin_local_row;
                block_id[global_row][col] = (origin_global_row, origin_col);
                borders_matrix[global_row][col] =
                    *resolved_borders.get(&(origin_global_row, origin_col)).unwrap_or(&Borders::ALL);
            }
        }
    }

    let border_grid = BorderGrid::new(total_rows, column_count, block_id.clone(), borders_matrix);

    // Which horizontal line indices are section boundaries (between two
    // *actually present* sections), and thus use the section-separator
    // glyph set instead of the regular one.
    let separator_lines: std::collections::HashSet<usize> =
        infos.iter().skip(1).map(|info| info.row_offset).collect();
    let regular_chars = table.border_preset.chars;
    let section_separator_chars = table.border_preset.section_separator_chars();

    let mut lines = Vec::new();

    for i in 0..=total_rows {
        let chars_for_line =
            if separator_lines.contains(&i) { &section_separator_chars } else { &regular_chars };
        let line = render_horizontal_line(&border_grid, chars_for_line, i, column_count, &widths);
        if !line.trim().is_empty() {
            lines.push(line);
        }

        if i < total_rows {
            lines.push(render_content_line(
                &border_grid,
                &regular_chars,
                i,
                column_count,
                &widths,
                &block_id,
                &resolved_cells,
            ));
        }
    }

    lines.join("\n")
}

/// Resolves `col`'s effective `min_width`/`width`/`max_width`: a
/// section-scoped [`Column`] override beats the table-wide one, same
/// specificity order as style/borders. Unlike those, though, two
/// *different* sections setting conflicting values for the same column is
/// a programming error rather than a cascade — a column's width is always
/// shared across the whole table, computed once, so "header says 40, body
/// says 50" has no sensible resolution. Panics in that case, naming the
/// conflicting sources; set it table-wide via `t.column(col)` instead.
fn resolve_column_constraints(table: &Table, infos: &[SectionInfo<'_>], column_count: usize) -> Vec<ColumnConstraints> {
    (0..column_count)
        .map(|col| {
            let table_column = table.columns.get(&col);
            ColumnConstraints {
                min_width: resolve_one_constraint(col, "min_width", table_column, infos, |c| c.min_width),
                width: resolve_one_constraint(col, "width", table_column, infos, |c| c.width),
                max_width: resolve_one_constraint(col, "max_width", table_column, infos, |c| c.max_width),
            }
        })
        .collect()
}

fn resolve_one_constraint(
    col: usize,
    name: &str,
    table_column: Option<&Column>,
    infos: &[SectionInfo<'_>],
    get: impl Fn(&Column) -> Option<usize>,
) -> Option<usize> {
    let mut section_value: Option<usize> = None;
    for info in infos {
        if let Some(value) = info.section.columns.get(&col).and_then(&get) {
            if let Some(existing) = section_value
                && existing != value
            {
                panic!(
                    "column {col} has conflicting `{name}` overrides across sections ({existing} vs \
                     {value}) — a column's width is shared across the whole table, so set `{name}` \
                     table-wide via `t.column({col})` instead of per-section"
                );
            }
            section_value = Some(value);
        }
    }
    section_value.or_else(|| table_column.and_then(&get))
}

/// Applies each column's resolved constraints on top of its natural width:
/// an explicit `width` wins outright (forcing truncation/padding to that
/// exact width later); otherwise the natural width is clamped between
/// `min_width` and `max_width`.
fn apply_column_constraints(natural_widths: Vec<usize>, constraints: &[ColumnConstraints]) -> Vec<usize> {
    natural_widths
        .into_iter()
        .zip(constraints)
        .map(|(natural, c)| {
            if let Some(width) = c.width {
                return width;
            }
            let mut width = natural;
            if let Some(max_width) = c.max_width {
                width = width.min(max_width);
            }
            if let Some(min_width) = c.min_width {
                width = width.max(min_width);
            }
            width
        })
        .collect()
}

/// Finds the column marked `flex` (table-wide or in any section), if any.
/// Panics if more than one column is — `cap_width`'s leftover space has no
/// sensible way to split between two flex columns yet, so only one is
/// supported per table.
fn resolve_flex_column(table: &Table, infos: &[SectionInfo<'_>], column_count: usize) -> Option<usize> {
    let flex_columns: Vec<usize> = (0..column_count)
        .filter(|&col| {
            table.columns.get(&col).is_some_and(|c| c.flex)
                || infos.iter().any(|info| info.section.columns.get(&col).is_some_and(|c| c.flex))
        })
        .collect();
    match flex_columns.as_slice() {
        [] => None,
        [col] => Some(*col),
        cols => panic!(
            "only one column may be marked `flex`, but columns {cols:?} all are — `cap_width`'s \
             leftover space has no sensible way to split between multiple flex columns yet"
        ),
    }
}

/// Grows or shrinks the flex column to take exactly whatever width is left
/// after every other column (plus separators) fits `target`, clamped to its
/// own `min_width`/`max_width` if set (and left untouched if it has an
/// explicit `width`, which already pins it). Runs before [`shrink_to_fit`],
/// which still applies afterward for any overflow the clamp couldn't
/// absorb — e.g. if the flex column's `min_width` doesn't fit alongside
/// every other column's own floor.
fn apply_flex_width(widths: &mut [usize], constraints: &[ColumnConstraints], flex_col: usize, target: usize, column_count: usize) {
    if constraints[flex_col].width.is_some() {
        return;
    }
    let others: usize = widths.iter().enumerate().filter(|&(col, _)| col != flex_col).map(|(_, &w)| w).sum();
    let separators = column_count.saturating_sub(1);
    let remaining = target.saturating_sub(others + separators);

    let mut width = remaining.max(1);
    if let Some(max_width) = constraints[flex_col].max_width {
        width = width.min(max_width);
    }
    if let Some(min_width) = constraints[flex_col].min_width {
        width = width.max(min_width);
    }
    widths[flex_col] = width;
}

/// Shrinks `widths` in place, repeatedly taking one character off the
/// currently-widest column that still has room to shrink, until their sum
/// (plus one separator character between adjacent columns) fits `target` or
/// every column has hit its floor — an explicit `width` column never
/// shrinks (it's fixed), others stop at their `min_width` (default `1`, so
/// no column disappears entirely).
fn shrink_to_fit(widths: &mut [usize], constraints: &[ColumnConstraints], target: usize) {
    let floors: Vec<usize> = constraints
        .iter()
        .enumerate()
        .map(|(col, c)| if c.width.is_some() { widths[col] } else { c.min_width.unwrap_or(1) })
        .collect();

    let total_width = |widths: &[usize]| widths.iter().sum::<usize>() + widths.len().saturating_sub(1);

    while total_width(widths) > target {
        let Some((widest, _)) =
            widths.iter().enumerate().filter(|&(col, &w)| w > floors[col]).max_by_key(|&(_, &w)| w)
        else {
            break; // every column is already at its floor; over budget stays over budget
        };
        widths[widest] -= 1;
    }
}

/// Rewrites every resolved cell's content to fit its column's final,
/// resolved width, truncating with `ellipsis` where needed. For a
/// colspanning cell, sums the widths of every column it spans (plus
/// separators) instead of just its origin column's width — a spanning cell
/// gets the room its whole span provides, not just one column's share of it.
fn truncate_resolved_cells(
    resolved_cells: &mut HashMap<(usize, usize), ResolvedCell>,
    widths: &[usize],
    column_count: usize,
    ellipsis: &str,
) {
    for (&(_, col), rc) in resolved_cells.iter_mut() {
        let available = merged_width(widths, col, rc.colspan, column_count);
        rc.content = truncate_with_ellipsis(&rc.content, available, ellipsis);
    }
}

/// The rendered width available to a cell starting at `col` and spanning
/// `colspan` columns: the sum of those columns' widths, plus one separator
/// character for each internal boundary between them.
fn merged_width(widths: &[usize], col: usize, colspan: usize, column_count: usize) -> usize {
    widths[col..(col + colspan).min(column_count)].iter().sum::<usize>() + (colspan - 1)
}

fn render_horizontal_line(
    border_grid: &BorderGrid,
    chars: &BorderChars,
    i: usize,
    column_count: usize,
    widths: &[usize],
) -> String {
    let mut line = String::new();
    if border_grid.glyph(chars, i, 0) != ' ' {
        line.push(border_grid.glyph(chars, i, 0));
    }
    for (col, &width) in widths.iter().enumerate().take(column_count) {
        let segment_char = if border_grid.horizontal_segment(i, col) { chars.horizontal } else { ' ' };
        line.push_str(&segment_char.to_string().repeat(width));
        let glyph = border_grid.glyph(chars, i, col + 1);
        if glyph != ' ' || col + 1 < column_count {
            line.push(glyph);
        }
    }
    line
}

fn render_content_line(
    border_grid: &BorderGrid,
    chars: &BorderChars,
    global_row: usize,
    column_count: usize,
    widths: &[usize],
    block_id: &[Vec<(usize, usize)>],
    resolved_cells: &HashMap<(usize, usize), ResolvedCell>,
) -> String {
    let mut line = String::new();
    if border_grid.vertical_segment(global_row, 0) {
        line.push(chars.vertical);
    }

    let mut col = 0;
    while col < column_count {
        let (origin_row, origin_col) = block_id[global_row][col];
        let rc = resolved_cells
            .get(&(origin_row, origin_col))
            .expect("every grid position must map to a resolved origin cell");

        let width = merged_width(widths, col, rc.colspan, column_count);

        let text = if global_row == origin_row { rc.content.as_str() } else { "" };
        let padded = pad_align(text, width, rc.style.resolved_align());
        line.push_str(&rc.style.render(&padded));

        col += rc.colspan;
        if col <= column_count {
            let has_border = border_grid.vertical_segment(global_row, col);
            if has_border {
                line.push(chars.vertical);
            } else if col < column_count {
                line.push(' ');
            }
        }
    }

    line
}

fn pad_align(text: &str, width: usize, align: Align) -> String {
    let content_width = text.width();
    if content_width >= width {
        return text.to_string();
    }
    let total_pad = width - content_width;
    match align {
        Align::Left => format!("{text}{}", " ".repeat(total_pad)),
        Align::Right => format!("{}{text}", " ".repeat(total_pad)),
        Align::Center => {
            let left = total_pad / 2;
            let right = total_pad - left;
            format!("{}{text}{}", " ".repeat(left), " ".repeat(right))
        }
    }
}
