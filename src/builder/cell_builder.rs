//! Per-cell DSL: colspan/rowspan, style, and border overrides.

use crate::border::Borders;
use crate::model::cell::Cell;
use crate::style::{Align, Style};

/// Closure-DSL builder for a single [`Cell`], passed to
/// [`crate::builder::row_builder::RowBuilder::cell`].
pub struct CellBuilder<'a> {
    pub(crate) cell: &'a mut Cell,
}

impl CellBuilder<'_> {
    /// Sets how many columns this cell spans (merging it with the columns
    /// to its right). Must be `>= 1`.
    pub fn colspan(&mut self, colspan: usize) -> &mut Self {
        assert!(colspan >= 1, "colspan must be at least 1, got {colspan}");
        self.cell.colspan = colspan;
        self
    }

    /// Sets how many rows this cell spans (merging it with the rows below
    /// it). Must be `>= 1`.
    pub fn rowspan(&mut self, rowspan: usize) -> &mut Self {
        assert!(rowspan >= 1, "rowspan must be at least 1, got {rowspan}");
        self.cell.rowspan = rowspan;
        self
    }

    /// Sets this cell's horizontal alignment (merged into its style).
    pub fn align(&mut self, align: Align) -> &mut Self {
        self.cell.style = self.cell.style.align(align);
        self
    }

    /// Sets this cell's style override (wins over row/column/section/table
    /// styles for any field it sets).
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.cell.style = style;
        self
    }

    /// Sets this cell's border-side override.
    pub fn cell_borders(&mut self, borders: Borders) -> &mut Self {
        self.cell.borders = Some(borders);
        self
    }
}
