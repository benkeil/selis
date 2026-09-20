//! Per-column DSL: style/border overrides, scoped to a single section.

use crate::border::Borders;
use crate::model::column::Column;
use crate::style::{Align, Style};

/// Closure-DSL builder for one [`Column`] override, passed to
/// [`crate::builder::section_builder::SectionBuilder::column`].
pub struct ColumnBuilder<'a> {
    pub(crate) column: &'a mut Column,
}

impl ColumnBuilder<'_> {
    /// Sets this column's horizontal alignment (merged into its style).
    pub fn align(&mut self, align: Align) -> &mut Self {
        self.column.style = self.column.style.align(align);
        self
    }

    /// Sets this column's style override.
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.column.style = style;
        self
    }

    /// Sets this column's border-side override.
    pub fn cell_borders(&mut self, borders: Borders) -> &mut Self {
        self.column.borders = Some(borders);
        self
    }
}
