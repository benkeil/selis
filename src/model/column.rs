//! Per-column style/border overrides, scoped to a single section.

use crate::border::Borders;
use crate::style::Style;

/// Style/border overrides for one column, scoped to the
/// [`crate::model::section::Section`] (header/body/footer) it was defined
/// in — matching the Mordant DSL's `column(index) { ... }` blocks, which are
/// nested inside `header`/`body`/`footer`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Column {
    pub style: Style,
    pub borders: Option<Borders>,
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
}
