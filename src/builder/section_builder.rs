//! Per-section (header/body/footer) DSL: rows, per-column overrides, and
//! section-wide style/border overrides.

use crate::border::Borders;
use crate::builder::column_builder::ColumnBuilder;
use crate::builder::row_builder::RowBuilder;
use crate::model::cell::Cell;
use crate::model::row::Row;
use crate::model::table::{SectionKind, Table};
use crate::style::Style;

/// Closure-DSL builder for one [`crate::model::section::Section`] (header,
/// body, or footer), passed to [`crate::builder::table_builder::TableBuilder::header`]
/// / `body` / `footer`.
pub struct SectionBuilder<'a> {
    pub(crate) table: &'a mut Table,
    pub(crate) kind: SectionKind,
    /// Set by [`Self::row_styles`]: alternating styles applied to each row
    /// added afterwards (by position within this section), matching the
    /// Mordant DSL's `rowStyles(...)`. An explicit `.style(...)` set on a
    /// row via [`Self::row_with`] takes precedence over its zebra style.
    zebra_styles: Vec<Style>,
}

impl<'a> SectionBuilder<'a> {
    pub(crate) fn new(table: &'a mut Table, kind: SectionKind) -> Self {
        SectionBuilder { table, kind, zebra_styles: Vec::new() }
    }

    /// Sets this section's style override.
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.table.section_mut(self.kind).style = style;
        self
    }

    /// Sets this section's border-side override.
    pub fn cell_borders(&mut self, borders: Borders) -> &mut Self {
        self.table.section_mut(self.kind).borders = Some(borders);
        self
    }

    /// Sets alternating (zebra-striping) styles applied to each row added
    /// afterwards, cycling through `styles` by row position within this
    /// section. Any style an individual row sets explicitly (via
    /// [`Self::row_with`]) wins over its zebra style.
    pub fn row_styles(&mut self, styles: impl IntoIterator<Item = Style>) -> &mut Self {
        self.zebra_styles = styles.into_iter().collect();
        self
    }

    /// Overrides style/border settings for one column (by 0-based index),
    /// scoped to this section.
    pub fn column(&mut self, index: usize, f: impl FnOnce(&mut ColumnBuilder)) -> &mut Self {
        let column = self.table.section_mut(self.kind).column_mut(index);
        f(&mut ColumnBuilder { column });
        self
    }

    /// Appends a row of plain-text cells.
    pub fn row<I, C>(&mut self, cells: I) -> &mut Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Cell>,
    {
        let row = Row::from_cells(cells);
        self.push_row(row);
        self
    }

    /// Appends a row built with per-cell/per-row customization (colspan,
    /// rowspan, style, borders).
    pub fn row_with(&mut self, f: impl FnOnce(&mut RowBuilder)) -> &mut Self {
        let mut row = Row::new();
        f(&mut RowBuilder { row: &mut row });
        self.push_row(row);
        self
    }

    fn push_row(&mut self, mut row: Row) {
        if !self.zebra_styles.is_empty() {
            let row_index = self
                .table
                .section(self.kind)
                .map(|s| s.rows.len())
                .unwrap_or(0);
            let zebra = self.zebra_styles[row_index % self.zebra_styles.len()];
            row.style = Style::cascade([Some(&zebra), Some(&row.style)]);
        }
        self.table.push_row(self.kind, row);
    }
}
