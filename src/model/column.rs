//! Per-column style/border/width overrides, either scoped to a single
//! section or table-wide (spanning all sections).

use crate::border::Borders;
use crate::style::Style;

/// Style/border/width overrides for one column. Used both scoped to a
/// single [`crate::model::section::Section`] (header/body/footer) —
/// matching the Mordant DSL's `column(index) { ... }` blocks nested inside
/// `header`/`body`/`footer` — and table-wide via
/// [`crate::model::table::Table::columns`], an "Excel-style" whole-column
/// selection that applies across all sections but is less specific than a
/// section-scoped override for the same index.
///
/// Since a column's rendered width is always computed once, shared across
/// header/body/footer, setting `min_width`/`width`/`max_width` on the same
/// column from two *different* sections with conflicting values is a
/// programming error — the renderer panics rather than silently picking one
/// (see [`crate::render::renderer::render`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Column {
    pub style: Style,
    pub borders: Option<Borders>,
    /// The column never renders narrower than this, even if its content is
    /// narrower.
    pub min_width: Option<usize>,
    /// Forces the column to render at exactly this width: narrower content
    /// is padded, wider content is truncated (with a trailing
    /// [`crate::model::table::Table::ellipsis`]). Ignores `min_width`/
    /// `max_width` when set.
    pub width: Option<usize>,
    /// The column never renders wider than this; content that doesn't fit
    /// is truncated (with a trailing [`crate::model::table::Table::ellipsis`]).
    pub max_width: Option<usize>,
    /// Marks this column as the one that absorbs whatever width is left
    /// over after every other column, once [`crate::model::table::Table::cap_width`]
    /// is set — it grows to fill slack, or shrinks (truncating its content)
    /// if there isn't any. A no-op without `cap_width`. Only one column per
    /// table may be marked `flex`; the renderer panics if more than one is.
    pub flex: bool,
}

impl Column {
    /// Creates a column with no overrides.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets this column's style override.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets this column's border-side override.
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = Some(borders);
        self
    }

    /// Sets this column's minimum width.
    pub fn min_width(mut self, min_width: usize) -> Self {
        self.min_width = Some(min_width);
        self
    }

    /// Sets this column's exact, fixed width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets this column's maximum width; content wider than this gets
    /// truncated (with a trailing ellipsis) to fit.
    pub fn max_width(mut self, max_width: usize) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Marks this column as the one that absorbs whatever width is left
    /// over after every other column, once `cap_width` is set: it grows to
    /// fill slack, or shrinks (truncating its content) if there isn't any.
    /// A no-op without `cap_width`. Only one column per table may be marked
    /// `flex`; the renderer panics if more than one is.
    pub fn flex(mut self) -> Self {
        self.flex = true;
        self
    }
}
