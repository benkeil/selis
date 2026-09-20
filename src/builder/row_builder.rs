//! Per-row DSL: adding plain or per-cell-customized cells, and row-level
//! style/border overrides.

use crate::border::Borders;
use crate::builder::cell_builder::CellBuilder;
use crate::model::cell::Cell;
use crate::model::row::Row;
use crate::style::Style;

/// Closure-DSL builder for a single [`Row`], passed to
/// [`crate::builder::section_builder::SectionBuilder::row_with`].
pub struct RowBuilder<'a> {
    pub(crate) row: &'a mut Row,
}

impl RowBuilder<'_> {
    /// Appends a run of plain-text cells.
    pub fn cells<I, C>(&mut self, cells: I) -> &mut Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Cell>,
    {
        self.row.cells.extend(cells.into_iter().map(Into::into));
        self
    }

    /// Appends a single cell with per-cell customization (colspan, rowspan,
    /// style, borders), e.g. `r.cell("Percent Change", |c| { c.colspan(2); })`.
    pub fn cell(&mut self, content: impl Into<String>, f: impl FnOnce(&mut CellBuilder)) -> &mut Self {
        let mut cell = Cell::new(content);
        f(&mut CellBuilder { cell: &mut cell });
        self.row.cells.push(cell);
        self
    }

    /// Sets this row's style override.
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.row.style = style;
        self
    }

    /// Sets this row's border-side override.
    pub fn cell_borders(&mut self, borders: Borders) -> &mut Self {
        self.row.borders = Some(borders);
        self
    }
}
