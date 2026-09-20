//! Resolves colspan/rowspan into a rectangular grid, matching each grid
//! position to either the *origin* cell that starts there, or the origin
//! coordinates of the merged cell that covers it (a "spanned" position).
//!
//! This is the same algorithm browsers use to lay out `<table>` `colspan`/
//! `rowspan`: cells are placed left-to-right, top-to-bottom, skipping any
//! grid position already claimed by an earlier row's `rowspan`.

use crate::model::section::Section;

/// One position in the resolved grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridSlot {
    /// The top-left grid position of a cell — `source_row`/`source_cell`
    /// index back into [`Section::rows`] / [`crate::model::row::Row::cells`].
    Origin {
        source_row: usize,
        /// `None` for cells auto-padded because their declared row had
        /// fewer cells than the columns actually free at that position
        /// (accounting for rowspans carried over from earlier rows) —
        /// these always render as an empty (`""`), unstyled, colspan/
        /// rowspan-1 cell.
        source_cell: Option<usize>,
        colspan: usize,
        rowspan: usize,
    },
    /// A grid position covered by a colspan/rowspan from the cell at
    /// `(origin_row, origin_col)`.
    Spanned { origin_row: usize, origin_col: usize },
}

impl GridSlot {
    /// The `(row, col)` of the origin cell that owns this grid position
    /// (itself, if this slot *is* the origin).
    pub fn origin(&self, row: usize, col: usize) -> (usize, usize) {
        match *self {
            GridSlot::Origin { .. } => (row, col),
            GridSlot::Spanned { origin_row, origin_col } => (origin_row, origin_col),
        }
    }
}

/// A resolved `row_count x column_count` grid for one [`Section`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub row_count: usize,
    pub column_count: usize,
    /// `slots[row][col]`, always fully populated (`row_count x column_count`).
    pub slots: Vec<Vec<GridSlot>>,
}

/// Resolves `section`'s rows into a [`Grid`] with `column_count` columns.
///
/// # Panics
/// Panics if a row's cells (after accounting for columns already claimed by
/// a previous row's `rowspan`) don't fit within `column_count` — this can
/// happen even when [`crate::model::table::Table::push_row`]'s own
/// validation passed, since that validation doesn't know about rowspans
/// from earlier rows.
pub fn resolve(section: &Section, column_count: usize) -> Grid {
    let row_count = section.rows.len();
    let mut slots: Vec<Vec<Option<GridSlot>>> = vec![vec![None; column_count]; row_count];

    for (row_idx, row) in section.rows.iter().enumerate() {
        let mut col_cursor = 0usize;
        for (cell_idx, cell) in row.cells.iter().enumerate() {
            while col_cursor < column_count && slots[row_idx][col_cursor].is_some() {
                col_cursor += 1;
            }
            assert!(
                col_cursor < column_count,
                "row {row_idx} overflows the table's {column_count} columns once rowspans from \
                 earlier rows are accounted for (cell {cell_idx}: {:?})",
                cell.content
            );

            let origin_col = col_cursor;
            for r_off in 0..cell.rowspan {
                let r = row_idx + r_off;
                if r >= row_count {
                    break;
                }
                for c_off in 0..cell.colspan {
                    let c = origin_col + c_off;
                    if c >= column_count {
                        break;
                    }
                    slots[r][c] = Some(if r_off == 0 && c_off == 0 {
                        GridSlot::Origin {
                            source_row: row_idx,
                            source_cell: Some(cell_idx),
                            colspan: cell.colspan,
                            rowspan: cell.rowspan,
                        }
                    } else {
                        GridSlot::Spanned {
                            origin_row: row_idx,
                            origin_col,
                        }
                    });
                }
            }
            col_cursor += cell.colspan;
        }

        // Auto-pad any positions in this row that are still free (not
        // covered by one of this row's own cells, and not carried over from
        // an earlier row's rowspan) with an empty, unspanned cell.
        for slot in slots[row_idx].iter_mut().take(column_count) {
            if slot.is_none() {
                *slot = Some(GridSlot::Origin {
                    source_row: row_idx,
                    source_cell: None,
                    colspan: 1,
                    rowspan: 1,
                });
            }
        }
    }

    let slots = slots
        .into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.into_iter()
                .enumerate()
                .map(|(col_idx, slot)| {
                    slot.unwrap_or_else(|| {
                        panic!("internal error: grid position ({row_idx}, {col_idx}) was never filled")
                    })
                })
                .collect()
        })
        .collect();

    Grid {
        row_count,
        column_count,
        slots,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::cell::Cell;
    use crate::model::row::Row;

    #[test]
    fn plain_grid_has_only_origins() {
        let mut section = Section::new();
        section.rows.push(Row::from_cells(["a", "b", "c"]));
        section.rows.push(Row::from_cells(["d", "e", "f"]));

        let grid = resolve(&section, 3);

        for row in 0..2 {
            for col in 0..3 {
                assert!(matches!(grid.slots[row][col], GridSlot::Origin { .. }));
            }
        }
    }

    #[test]
    fn colspan_creates_spanned_positions_to_the_right() {
        let mut section = Section::new();
        let mut row = Row::new();
        row.push(Cell::new("wide").colspan(2));
        row.push(Cell::new("normal"));
        section.rows.push(row);

        let grid = resolve(&section, 3);

        assert!(matches!(
            grid.slots[0][0],
            GridSlot::Origin { colspan: 2, .. }
        ));
        assert_eq!(grid.slots[0][1], GridSlot::Spanned { origin_row: 0, origin_col: 0 });
        assert!(matches!(grid.slots[0][2], GridSlot::Origin { .. }));
    }

    #[test]
    fn rowspan_creates_spanned_positions_below_and_shifts_next_rows_cells() {
        let mut section = Section::new();

        let mut row0 = Row::new();
        row0.push(Cell::new("tall").rowspan(2));
        row0.push(Cell::new("r0c1"));
        section.rows.push(row0);

        let mut row1 = Row::new();
        row1.push(Cell::new("r1c1"));
        section.rows.push(row1);

        let grid = resolve(&section, 2);

        assert!(matches!(grid.slots[0][0], GridSlot::Origin { rowspan: 2, .. }));
        assert_eq!(grid.slots[1][0], GridSlot::Spanned { origin_row: 0, origin_col: 0 });
        // row 1's only declared cell ends up in column 1, since column 0 was
        // already claimed by the rowspan.
        match &grid.slots[1][1] {
            GridSlot::Origin { source_row, source_cell, .. } => {
                assert_eq!(*source_row, 1);
                assert_eq!(*source_cell, Some(0));
            }
            other => panic!("expected an Origin slot, got {other:?}"),
        }
    }

    #[test]
    fn short_row_is_auto_padded_with_empty_unspanned_cells() {
        let mut section = Section::new();
        section.rows.push(Row::from_cells(["a", "b"])); // only 2 of 4 columns declared

        let grid = resolve(&section, 4);

        for col in 2..4 {
            match &grid.slots[0][col] {
                GridSlot::Origin { source_cell: None, colspan: 1, rowspan: 1, .. } => {}
                other => panic!("expected an auto-padded empty Origin slot at col {col}, got {other:?}"),
            }
        }
    }

    #[test]
    fn auto_padding_accounts_for_rowspan_carried_over_from_an_earlier_row() {
        // Row 0: a rowspan(3) cell in column 0, plus 3 more cells (column
        // count 4). Row 1 only declares 3 cells (columns 1-3, since column 0
        // is still claimed by row 0's rowspan) - it must NOT be auto-padded
        // with a 4th cell that would overflow the table.
        let mut section = Section::new();
        let mut row0 = Row::new();
        row0.push(Cell::new("tall").rowspan(3));
        row0.push(Cell::new("r0c1"));
        row0.push(Cell::new("r0c2"));
        row0.push(Cell::new("r0c3"));
        section.rows.push(row0);
        section.rows.push(Row::from_cells(["r1c1", "r1c2", "r1c3"]));
        section.rows.push(Row::from_cells(["r2c1", "r2c2", "r2c3"]));

        let grid = resolve(&section, 4);

        assert_eq!(grid.slots[1][0], GridSlot::Spanned { origin_row: 0, origin_col: 0 });
        assert_eq!(grid.slots[2][0], GridSlot::Spanned { origin_row: 0, origin_col: 0 });
        for row in 1..3 {
            for col in 1..4 {
                assert!(matches!(
                    grid.slots[row][col],
                    GridSlot::Origin { source_cell: Some(_), .. }
                ));
            }
        }
    }

    #[test]
    fn colspan_and_rowspan_combine() {
        let mut section = Section::new();
        let mut row0 = Row::new();
        row0.push(Cell::new("big").colspan(2).rowspan(2));
        section.rows.push(row0);
        section.rows.push(Row::new());

        let grid = resolve(&section, 2);

        assert!(matches!(
            grid.slots[0][0],
            GridSlot::Origin { colspan: 2, rowspan: 2, .. }
        ));
        assert_eq!(grid.slots[0][1], GridSlot::Spanned { origin_row: 0, origin_col: 0 });
        assert_eq!(grid.slots[1][0], GridSlot::Spanned { origin_row: 0, origin_col: 0 });
        assert_eq!(grid.slots[1][1], GridSlot::Spanned { origin_row: 0, origin_col: 0 });
    }
}
