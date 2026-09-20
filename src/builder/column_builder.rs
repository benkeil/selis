//! Per-column DSL: style/border overrides, scoped to a single section.

use crate::border::Borders;
use crate::model::column::Column;
use crate::model::max_width::MaxWidth;
use crate::style::{Align, Color, Style};

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

    /// Sets this column's foreground (text) color (merged into its style).
    pub fn fg(&mut self, color: Color) -> &mut Self {
        self.column.style = self.column.style.fg(color);
        self
    }

    /// Sets this column's background color (merged into its style).
    pub fn bg(&mut self, color: Color) -> &mut Self {
        self.column.style = self.column.style.bg(color);
        self
    }

    /// Enables bold text for this column (merged into its style).
    pub fn bold(&mut self) -> &mut Self {
        self.column.style = self.column.style.bold();
        self
    }

    /// Enables italic text for this column (merged into its style).
    pub fn italic(&mut self) -> &mut Self {
        self.column.style = self.column.style.italic();
        self
    }

    /// Enables underlined text for this column (merged into its style).
    pub fn underline(&mut self) -> &mut Self {
        self.column.style = self.column.style.underline();
        self
    }

    /// Sets this column's style override.
    ///
    /// Merges onto whatever was already set on this builder (e.g. via
    /// [`Self::align`]), so call order doesn't matter. Pass a style marked
    /// [`Style::exact`] to hard-reset everything set so far instead.
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.column.style = if style.exact { style } else { self.column.style.merge(&style) };
        self
    }

    /// Sets this column's border-side override.
    pub fn cell_borders(&mut self, borders: Borders) -> &mut Self {
        self.column.borders = Some(borders);
        self
    }

    /// Sets this column's maximum width; content wider than this gets
    /// truncated (with a trailing `...`) to fit, e.g.
    /// `c.max_width(20);` or, for the (not yet implemented) auto mode,
    /// `c.max_width(MaxWidth::Auto);`.
    pub fn max_width(&mut self, max_width: impl Into<MaxWidth>) -> &mut Self {
        self.column.max_width = Some(max_width.into());
        self
    }
}
