//! Per-column style/border overrides, either scoped to a single section or
//! table-wide (spanning all sections).

use crate::border::Borders;
use crate::model::max_width::MaxWidth;
use crate::style::Style;

/// Style/border overrides for one column. Used both scoped to a single
/// [`crate::model::section::Section`] (header/body/footer) — matching the
/// Mordant DSL's `column(index) { ... }` blocks nested inside
/// `header`/`body`/`footer` — and table-wide via
/// [`crate::model::table::Table::columns`], an "Excel-style" whole-column
/// selection that applies across all sections but is less specific than a
/// section-scoped override for the same index.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Column {
    pub style: Style,
    pub borders: Option<Borders>,
    pub max_width: Option<MaxWidth>,
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

    /// Sets this column's maximum width; content wider than this gets
    /// truncated (with a trailing `...`) to fit.
    pub fn max_width(mut self, max_width: impl Into<MaxWidth>) -> Self {
        self.max_width = Some(max_width.into());
        self
    }
}
