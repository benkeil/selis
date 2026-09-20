//! Per-cell DSL: colspan/rowspan, style, and border overrides.

use crate::border::Borders;
use crate::model::cell::Cell;
use crate::style::{Align, Color, Style};
use crate::text::truncate_with_ellipsis;

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

    /// Sets this cell's foreground (text) color (merged into its style).
    pub fn fg(&mut self, color: Color) -> &mut Self {
        self.cell.style = self.cell.style.fg(color);
        self
    }

    /// Sets this cell's background color (merged into its style).
    pub fn bg(&mut self, color: Color) -> &mut Self {
        self.cell.style = self.cell.style.bg(color);
        self
    }

    /// Enables bold text for this cell (merged into its style).
    pub fn bold(&mut self) -> &mut Self {
        self.cell.style = self.cell.style.bold();
        self
    }

    /// Enables italic text for this cell (merged into its style).
    pub fn italic(&mut self) -> &mut Self {
        self.cell.style = self.cell.style.italic();
        self
    }

    /// Enables underlined text for this cell (merged into its style).
    pub fn underline(&mut self) -> &mut Self {
        self.cell.style = self.cell.style.underline();
        self
    }

    /// Sets this cell's style override (wins over row/column/section/table
    /// styles for any field it sets).
    ///
    /// Merges onto whatever was already set on this builder (e.g. via
    /// [`Self::align`]), so call order doesn't matter. Pass a style marked
    /// [`Style::exact`] to hard-reset everything set so far instead.
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.cell.style = if style.exact { style } else { self.cell.style.merge(&style) };
        self
    }

    /// Sets this cell's border-side override.
    pub fn cell_borders(&mut self, borders: Borders) -> &mut Self {
        self.cell.borders = Some(borders);
        self
    }

    /// Truncates this cell's content to `max_width` right now, appending
    /// `"..."` if it had to cut anything off. This directly replaces the
    /// cell's content rather than affecting layout later, so the column
    /// this cell ends up in is naturally sized around the truncated text.
    pub fn truncate(&mut self, max_width: usize) -> &mut Self {
        self.truncate_with(max_width, "...")
    }

    /// Like [`Self::truncate`], but with a custom `ellipsis` instead of the
    /// default `"..."` (e.g. `"…"`).
    pub fn truncate_with(&mut self, max_width: usize, ellipsis: &str) -> &mut Self {
        self.cell.content = truncate_with_ellipsis(&self.cell.content, max_width, ellipsis);
        self
    }
}
