//! A single table cell: its text content, span, and style/border overrides.

use crate::border::Borders;
use crate::style::Style;

/// A single cell of a [`crate::model::row::Row`].
///
/// `colspan`/`rowspan` let a cell merge with its neighbors, matching the
/// Mordant DSL's `columnSpan`/`rowSpan`. Both default to `1` (no merging).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub content: String,
    /// How many columns this cell spans (>= 1). `2` merges this cell with
    /// the column immediately to its right.
    pub colspan: usize,
    /// How many rows this cell spans (>= 1). `2` merges this cell with the
    /// row immediately below it.
    pub rowspan: usize,
    /// Style overrides for just this cell (merged on top of row/column/
    /// section/table styles during rendering).
    pub style: Style,
    /// Border-side overrides for just this cell. `None` means "inherit from
    /// row/column/section/table".
    pub borders: Option<Borders>,
    /// Set by [`Self::truncate`]: truncates this cell's content to this
    /// width *before* natural column widths are computed (unlike a column's
    /// `max_width`, which only clamps afterward), using the table's
    /// [`crate::model::table::Table::ellipsis`]. `None` leaves content as-is.
    pub truncate_to: Option<usize>,
}

impl Cell {
    /// Creates a new, unspanned (`colspan = rowspan = 1`) cell with the given
    /// text content and no style/border overrides.
    pub fn new(content: impl Into<String>) -> Self {
        Cell {
            content: content.into(),
            colspan: 1,
            rowspan: 1,
            style: Style::new(),
            borders: None,
            truncate_to: None,
        }
    }

    /// Creates an empty (`""`) cell, used to auto-pad rows that have fewer
    /// cells than the table's column count.
    pub fn empty() -> Self {
        Cell::new("")
    }

    /// Sets the number of columns this cell spans. Must be `>= 1`.
    pub fn colspan(mut self, colspan: usize) -> Self {
        assert!(colspan >= 1, "colspan must be at least 1, got {colspan}");
        self.colspan = colspan;
        self
    }

    /// Sets the number of rows this cell spans. Must be `>= 1`.
    pub fn rowspan(mut self, rowspan: usize) -> Self {
        assert!(rowspan >= 1, "rowspan must be at least 1, got {rowspan}");
        self.rowspan = rowspan;
        self
    }

    /// Sets this cell's style override.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets this cell's border-side override.
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = Some(borders);
        self
    }

    /// Truncates this cell's content to `max_width`, appending the table's
    /// [`crate::model::table::Table::ellipsis`] if it had to cut anything
    /// off. Applied before natural column widths are computed, so the
    /// column this cell ends up in is naturally sized around the truncated
    /// text — unlike a column's `max_width`, which only clamps afterward.
    /// Set `t.ellipsis(...)` for a custom ellipsis; it applies here too.
    pub fn truncate(mut self, max_width: usize) -> Self {
        self.truncate_to = Some(max_width);
        self
    }
}

impl From<&str> for Cell {
    fn from(value: &str) -> Self {
        Cell::new(value)
    }
}

impl From<String> for Cell {
    fn from(value: String) -> Self {
        Cell::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cell_defaults_to_unspanned() {
        let cell = Cell::new("hi");
        assert_eq!(cell.colspan, 1);
        assert_eq!(cell.rowspan, 1);
        assert_eq!(cell.content, "hi");
    }

    #[test]
    fn empty_cell_has_empty_content() {
        assert_eq!(Cell::empty().content, "");
    }

    #[test]
    #[should_panic(expected = "colspan must be at least 1")]
    fn colspan_zero_panics() {
        Cell::new("x").colspan(0);
    }

    #[test]
    fn truncate_stores_the_target_width_without_touching_content() {
        // The actual truncation happens at render time (it needs the
        // table's `ellipsis`, which doesn't exist yet at this point).
        let cell = Cell::new("a very long piece of text").truncate(10);
        assert_eq!(cell.content, "a very long piece of text");
        assert_eq!(cell.truncate_to, Some(10));
    }
}
