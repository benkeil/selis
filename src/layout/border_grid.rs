//! Merges neighboring cells' resolved [`Borders`] flags into the actual
//! box-drawing glyph to use at each grid vertex — the terminal-table
//! equivalent of CSS's `border-collapse`.
//!
//! Two adjacent cells only draw a border segment between them if at least
//! one of them requests it on the touching side, and never if they belong
//! to the same merged (colspan/rowspan) cell.

use crate::border::{BorderChars, Borders};

/// A resolved `row_count x column_count` grid of per-slot border requests,
/// ready to be turned into box-drawing glyphs.
///
/// Every grid slot (including ones covered by a colspan/rowspan) must carry
/// the *same* resolved [`Borders`] and block id as its cell's origin, so
/// internal edges of a merged cell are correctly suppressed.
pub struct BorderGrid {
    row_count: usize,
    column_count: usize,
    /// `block_id[row][col]` — the origin `(row, col)` of the cell occupying
    /// this slot (equal to `(row, col)` itself for unspanned/origin slots).
    block_id: Vec<Vec<(usize, usize)>>,
    /// `borders[row][col]` — the resolved border sides for the cell
    /// occupying this slot.
    borders: Vec<Vec<Borders>>,
}

impl BorderGrid {
    pub fn new(row_count: usize, column_count: usize, block_id: Vec<Vec<(usize, usize)>>, borders: Vec<Vec<Borders>>) -> Self {
        assert_eq!(block_id.len(), row_count);
        assert_eq!(borders.len(), row_count);
        for r in &block_id {
            assert_eq!(r.len(), column_count);
        }
        for r in &borders {
            assert_eq!(r.len(), column_count);
        }
        BorderGrid { row_count, column_count, block_id, borders }
    }

    /// Whether a vertical border segment is drawn at vertical line `j`
    /// (`0..=column_count`) along row `row_band` (`0..row_count`).
    pub(crate) fn vertical_segment(&self, row_band: usize, j: usize) -> bool {
        if row_band >= self.row_count {
            return false;
        }
        if j == 0 {
            self.borders[row_band][0].contains(Borders::LEFT)
        } else if j == self.column_count {
            self.borders[row_band][self.column_count - 1].contains(Borders::RIGHT)
        } else {
            let left = (row_band, j - 1);
            let right = (row_band, j);
            self.block_id[left.0][left.1] != self.block_id[right.0][right.1]
                && (self.borders[left.0][left.1].contains(Borders::RIGHT)
                    || self.borders[right.0][right.1].contains(Borders::LEFT))
        }
    }

    /// Whether a horizontal border segment is drawn at horizontal line `i`
    /// (`0..=row_count`) along column `col_band` (`0..column_count`).
    pub(crate) fn horizontal_segment(&self, i: usize, col_band: usize) -> bool {
        if col_band >= self.column_count {
            return false;
        }
        if i == 0 {
            self.borders[0][col_band].contains(Borders::TOP)
        } else if i == self.row_count {
            self.borders[self.row_count - 1][col_band].contains(Borders::BOTTOM)
        } else {
            let above = (i - 1, col_band);
            let below = (i, col_band);
            self.block_id[above.0][above.1] != self.block_id[below.0][below.1]
                && (self.borders[above.0][above.1].contains(Borders::BOTTOM)
                    || self.borders[below.0][below.1].contains(Borders::TOP))
        }
    }

    /// The `(up, right, down, left)` directions present at the grid vertex
    /// `(i, j)`, where `i` is `0..=row_count` and `j` is `0..=column_count`.
    pub fn vertex_directions(&self, i: usize, j: usize) -> (bool, bool, bool, bool) {
        let up = i > 0 && self.vertical_segment(i - 1, j);
        let down = i < self.row_count && self.vertical_segment(i, j);
        let left = j > 0 && self.horizontal_segment(i, j - 1);
        let right = j < self.column_count && self.horizontal_segment(i, j);
        (up, right, down, left)
    }

    /// The glyph to draw at vertex `(i, j)` using `chars`.
    pub fn glyph(&self, chars: &BorderChars, i: usize, j: usize) -> char {
        let (up, right, down, left) = self.vertex_directions(i, j);
        chars.junction(up, right, down, left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uniform_grid(row_count: usize, column_count: usize, borders: Borders) -> BorderGrid {
        let block_id = (0..row_count)
            .map(|r| (0..column_count).map(|c| (r, c)).collect())
            .collect();
        let borders_grid = (0..row_count).map(|_| vec![borders; column_count]).collect();
        BorderGrid::new(row_count, column_count, block_id, borders_grid)
    }

    #[test]
    fn full_grid_of_all_borders_draws_a_complete_box_with_crosses() {
        let grid = uniform_grid(2, 2, Borders::ALL);
        let chars = BorderChars::UTF8_SQUARE;

        assert_eq!(grid.glyph(&chars, 0, 0), '┌');
        assert_eq!(grid.glyph(&chars, 0, 2), '┐');
        assert_eq!(grid.glyph(&chars, 2, 0), '└');
        assert_eq!(grid.glyph(&chars, 2, 2), '┘');
        assert_eq!(grid.glyph(&chars, 1, 1), '┼');
        assert_eq!(grid.glyph(&chars, 0, 1), '┬');
        assert_eq!(grid.glyph(&chars, 1, 0), '├');
    }

    #[test]
    fn merged_cell_suppresses_its_internal_border() {
        // A single 1x2 merged cell (block (0,0) covers both columns): no
        // vertical segment should be drawn between them, even though both
        // slots request ALL borders.
        let block_id = vec![vec![(0, 0), (0, 0)]];
        let borders = vec![vec![Borders::ALL, Borders::ALL]];
        let grid = BorderGrid::new(1, 2, block_id, borders);
        let chars = BorderChars::UTF8_SQUARE;

        // top-middle and bottom-middle vertices should just be straight
        // horizontal lines (no down/up branch into the cell), since there's
        // no internal vertical border.
        assert_eq!(grid.glyph(&chars, 0, 1), '─');
        assert_eq!(grid.glyph(&chars, 1, 1), '─');
    }

    #[test]
    fn no_borders_is_all_spaces() {
        let grid = uniform_grid(2, 2, Borders::NONE);
        let chars = BorderChars::UTF8_SQUARE;
        for i in 0..=2 {
            for j in 0..=2 {
                assert_eq!(grid.glyph(&chars, i, j), ' ');
            }
        }
    }
}
