//! Computes per-column content widths across one or more resolved
//! [`Grid`](crate::layout::grid::Grid)s (header/body/footer share the same
//! column widths), taking colspan into account: a spanning cell only forces
//! its columns wider if its own content doesn't already fit in their
//! combined natural width.

use unicode_width::UnicodeWidthStr;

use crate::layout::grid::{Grid, GridSlot};
use crate::model::section::Section;

/// One (section, resolved grid) pair to consider together when computing
/// shared column widths.
pub struct SectionGrid<'a> {
    pub section: &'a Section,
    pub grid: &'a Grid,
}

/// Computes the content width of each of `column_count` columns, given all
/// of a table's sections and a `column_separator_width` (how many extra
/// character columns sit between two adjacent columns when they are *not*
/// merged by a colspan — e.g. `1` for a rendered border/space, `0` if
/// columns are drawn directly adjacent).
///
/// A spanning cell's own content width is compared against the *combined*
/// width of the columns it spans (their natural widths plus the separator
/// width(s) between them, which become available content space once the
/// internal separator is removed for the merge). If the spanning cell needs
/// more room than that, the extra width is distributed as evenly as
/// possible across the spanned columns.
pub fn compute_column_widths(
    sections: &[SectionGrid<'_>],
    column_count: usize,
    column_separator_width: usize,
) -> Vec<usize> {
    let mut widths = vec![0usize; column_count];

    // Pass 1: unspanned cells set a natural width floor per column.
    for sg in sections {
        for (row_idx, row) in sg.grid.slots.iter().enumerate() {
            for (col_idx, slot) in row.iter().enumerate() {
                if let GridSlot::Origin { source_row, source_cell, colspan: 1, .. } = slot {
                    let content = cell_content(sg.section, *source_row, *source_cell);
                    widths[col_idx] = widths[col_idx].max(content.width());
                    debug_assert_eq!(row_idx, *source_row);
                }
            }
        }
    }

    // Pass 2: spanning cells widen their spanned columns if needed.
    for sg in sections {
        for row in &sg.grid.slots {
            for (col_idx, slot) in row.iter().enumerate() {
                if let GridSlot::Origin { source_row, source_cell, colspan, .. } = slot {
                    if *colspan <= 1 {
                        continue;
                    }
                    let content = cell_content(sg.section, *source_row, *source_cell);
                    let needed = content.width();
                    let span_cols = col_idx..(col_idx + colspan).min(column_count);
                    let current: usize =
                        span_cols.clone().map(|c| widths[c]).sum::<usize>()
                            + span_cols.len().saturating_sub(1) * column_separator_width;
                    if current < needed {
                        distribute_extra(&mut widths, span_cols, needed - current);
                    }
                }
            }
        }
    }

    widths
}

/// The text content of the cell at `(source_row, source_cell)`, or `""` for
/// an auto-padded slot (`source_cell` is `None`).
fn cell_content(section: &Section, source_row: usize, source_cell: Option<usize>) -> &str {
    match source_cell {
        Some(idx) => &section.rows[source_row].cells[idx].content,
        None => "",
    }
}

/// Adds `extra` character columns of width across `span`, spread as evenly
/// as possible (columns later in the span get the remainder first).
fn distribute_extra(widths: &mut [usize], span: std::ops::Range<usize>, extra: usize) {
    let n = span.len();
    if n == 0 {
        return;
    }
    let base = extra / n;
    let remainder = extra % n;
    for (i, col) in span.enumerate() {
        let bonus = if i >= n - remainder { 1 } else { 0 };
        widths[col] += base + bonus;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::grid;
    use crate::model::cell::Cell;
    use crate::model::row::Row;
    use crate::model::section::Section;

    fn section_with_rows(rows: Vec<Row>) -> Section {
        let mut section = Section::new();
        section.rows = rows;
        section
    }

    #[test]
    fn unspanned_columns_take_their_widest_cell() {
        let section = section_with_rows(vec![
            Row::from_cells(["a", "bb", "ccc"]),
            Row::from_cells(["aaaa", "b", "c"]),
        ]);
        let grid = grid::resolve(&section, 3);
        let sections = [SectionGrid { section: &section, grid: &grid }];

        let widths = compute_column_widths(&sections, 3, 1);

        assert_eq!(widths, vec![4, 2, 3]);
    }

    #[test]
    fn spanning_cell_that_already_fits_does_not_widen_columns() {
        let mut row = Row::new();
        row.push(Cell::new("ab").colspan(2)); // needs 2, columns already give 1+1+separator(1)=3
        row.push(Cell::new("x"));
        let section = section_with_rows(vec![row, Row::from_cells(["a", "b", "c"])]);
        let grid = grid::resolve(&section, 3);
        let sections = [SectionGrid { section: &section, grid: &grid }];

        let widths = compute_column_widths(&sections, 3, 1);

        assert_eq!(widths, vec![1, 1, 1]);
    }

    #[test]
    fn spanning_cell_that_needs_more_room_distributes_the_deficit() {
        let mut row = Row::new();
        row.push(Cell::new("Percent Change").colspan(2)); // width 14
        row.push(Cell::new("x"));
        let section = section_with_rows(vec![row, Row::from_cells(["a", "b", "c"])]);
        let grid = grid::resolve(&section, 3);
        let sections = [SectionGrid { section: &section, grid: &grid }];

        let widths = compute_column_widths(&sections, 3, 1);

        // natural: [1, 1, 1]; span covers cols 0-1: current = 1+1+1(sep) = 3;
        // needed = 14; deficit = 11, split across 2 columns -> 6 and 5 (remainder to later column).
        assert_eq!(widths[0] + widths[1] + 1, 14);
        assert_eq!(widths[2], 1);
    }

    #[test]
    fn header_body_footer_share_the_same_column_widths() {
        let header = section_with_rows(vec![Row::from_cells(["H1", "H2"])]);
        let body = section_with_rows(vec![Row::from_cells(["a", "bbbbbb"])]);
        let header_grid = grid::resolve(&header, 2);
        let body_grid = grid::resolve(&body, 2);

        let sections = [
            SectionGrid { section: &header, grid: &header_grid },
            SectionGrid { section: &body, grid: &body_grid },
        ];

        let widths = compute_column_widths(&sections, 2, 1);
        assert_eq!(widths, vec![2, 6]);
    }
}
