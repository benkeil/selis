//! Turns a resolved [`Table`] model + [`crate::layout`] geometry into the
//! final ANSI string.

use std::collections::HashMap;

use unicode_width::UnicodeWidthStr;

use crate::border::{BorderChars, Borders};
use crate::layout::border_grid::BorderGrid;
use crate::layout::grid::{self, Grid, GridSlot};
use crate::layout::width::{self, SectionGrid};
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

/// A fully-resolved origin cell, ready to be rendered. Its `content` is
/// already truncated to fit any configured column-level `max_width` — no
/// further truncation happens at render time.
struct ResolvedCell {
    colspan: usize,
    content: String,
    style: Style,
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
    // Resolve each column's effective max width: a section-scoped override
    // (checked in header/body/footer order, so a later section wins if more
    // than one sets it) beats the table-wide one, matching the same
    // specificity order used for style/borders elsewhere in this function.
    // Applied below by truncating each cell's own content *before* widths
    // are computed, so the column ends up naturally sized to fit.
    let max_widths: Vec<Option<usize>> = (0..column_count)
        .map(|col| {
            let mut resolved = table.columns.get(&col).and_then(|c| c.max_width);
            for info in &infos {
                if let Some(max_width) = info.section.columns.get(&col).and_then(|c| c.max_width) {
                    resolved = Some(max_width);
                }
            }
            resolved.and_then(|max_width| max_width.resolved_width())
        })
        .collect();

    // Resolve style/borders/content for every origin cell, keyed by its
    // global (row, col).
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
                    let content = match style.resolved_case() {
                        Some(case) => case.apply(&content),
                        None => content,
                    };
                    let content = match max_widths[col] {
                        Some(max_width) => truncate_with_ellipsis(&content, max_width, "..."),
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

    // Column widths are computed from the already-resolved (truncated,
    // case-transformed) content, so a column's configured `max_width` is
    // fully baked in by the time layout happens.
    let content_overrides: HashMap<(usize, usize), usize> =
        resolved_cells.iter().map(|(&pos, rc)| (pos, rc.content.width())).collect();
    let widths =
        width::compute_column_widths(&section_grids, column_count, 1, &content_overrides);



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

        let merged_width =
            widths[col..(col + rc.colspan).min(column_count)].iter().sum::<usize>() + (rc.colspan - 1);

        let text = if global_row == origin_row { rc.content.as_str() } else { "" };
        let padded = pad_align(text, merged_width, rc.style.resolved_align());
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
