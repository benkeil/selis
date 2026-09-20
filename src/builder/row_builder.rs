//! Per-row DSL: adding plain or per-cell-customized cells, and row-level
//! style/border overrides.

use crate::border::Borders;
use crate::builder::cell_builder::CellBuilder;
use crate::model::cell::Cell;
use crate::model::row::Row;
use crate::style::{Align, Color, Style};

/// Closure-DSL builder for a single [`Row`], passed to
/// [`crate::builder::section_builder::SectionBuilder::row`].
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

    /// Appends a single cell that can be further customized by chaining
    /// [`CellBuilder`] setters (colspan, rowspan, align, style, borders),
    /// e.g. `r.cell("Percent Change").colspan(2).align(Align::Center);`.
    pub fn cell(&mut self, content: impl Into<String>) -> CellBuilder<'_> {
        self.row.cells.push(Cell::new(content));
        let cell = self.row.cells.last_mut().expect("just pushed a cell");
        CellBuilder { cell }
    }

    /// Sets this row's style override.
    ///
    /// Merges onto whatever was already set on this builder (e.g. via
    /// [`Self::align`]), so call order doesn't matter. Pass a style marked
    /// [`Style::exact`] to hard-reset everything set so far instead.
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.row.style = if style.exact { style } else { self.row.style.merge(&style) };
        self
    }

    /// Sets this row's default horizontal alignment (merged into the row's
    /// style). Wins over the section/column/table default for this row
    /// only; a cell's own `.align(...)` still overrides it.
    pub fn align(&mut self, align: Align) -> &mut Self {
        self.row.style = self.row.style.align(align);
        self
    }

    /// Sets this row's foreground (text) color (merged into its style).
    pub fn fg(&mut self, color: Color) -> &mut Self {
        self.row.style = self.row.style.fg(color);
        self
    }

    /// Sets this row's background color (merged into its style).
    pub fn bg(&mut self, color: Color) -> &mut Self {
        self.row.style = self.row.style.bg(color);
        self
    }

    /// Enables bold text for this row (merged into its style).
    pub fn bold(&mut self) -> &mut Self {
        self.row.style = self.row.style.bold();
        self
    }

    /// Enables italic text for this row (merged into its style).
    pub fn italic(&mut self) -> &mut Self {
        self.row.style = self.row.style.italic();
        self
    }

    /// Enables underlined text for this row (merged into its style).
    pub fn underline(&mut self) -> &mut Self {
        self.row.style = self.row.style.underline();
        self
    }

    /// Sets this row's border-side override.
    pub fn cell_borders(&mut self, borders: Borders) -> &mut Self {
        self.row.borders = Some(borders);
        self
    }
}
