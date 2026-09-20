//! Per-column DSL: style/border overrides, scoped to a single section.

use crate::border::Borders;
use crate::model::column::Column;
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

    /// Sets this column's minimum width; it never renders narrower than
    /// this, even if its content is narrower.
    pub fn min_width(&mut self, min_width: usize) -> &mut Self {
        self.column.min_width = Some(min_width);
        self
    }

    /// Forces this column to render at exactly this width: narrower content
    /// is padded, wider content is truncated (with a trailing ellipsis).
    /// Ignores `min_width`/`max_width` when set.
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.column.width = Some(width);
        self
    }

    /// Sets this column's maximum width; content wider than this gets
    /// truncated (with a trailing ellipsis) to fit, e.g. `c.max_width(20);`.
    pub fn max_width(&mut self, max_width: usize) -> &mut Self {
        self.column.max_width = Some(max_width);
        self
    }

    /// Marks this column as the one that absorbs whatever width is left
    /// over after every other column, once `cap_width` is set: it grows to
    /// fill slack, or shrinks (truncating its content) if there isn't any.
    /// A no-op without `cap_width`. Only one column per table may be marked
    /// `flex`; the renderer panics if more than one is.
    pub fn flex(&mut self) -> &mut Self {
        self.column.flex = true;
        self
    }
}
