//! A section of a table: the header, body, or footer, each a list of rows
//! plus section-wide style/border overrides and scoped per-column overrides.

use std::collections::BTreeMap;

use crate::border::Borders;
use crate::model::column::Column;
use crate::model::row::Row;
use crate::style::Style;

/// One of a table's three sections (header, body, footer).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Section {
    pub rows: Vec<Row>,
    pub style: Style,
    pub borders: Option<Borders>,
    /// Per-column overrides scoped to this section, keyed by (0-based)
    /// column index, matching the Mordant DSL's `column(index) { ... }`
    /// blocks nested inside `header`/`body`/`footer`.
    pub columns: BTreeMap<usize, Column>,
}

impl Section {
    /// Creates an empty section (no rows, no overrides).
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets this section's style override.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets this section's border-side override.
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = Some(borders);
        self
    }

    /// Returns a mutable reference to the override for `column_index`,
    /// creating a default one if it doesn't exist yet.
    pub fn column_mut(&mut self, column_index: usize) -> &mut Column {
        self.columns.entry(column_index).or_default()
    }
}
