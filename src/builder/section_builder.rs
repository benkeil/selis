//! Per-section (header/body/footer) DSL: rows, per-column overrides, and
//! section-wide style/border overrides.

use crate::border::Borders;
use crate::builder::column_builder::ColumnBuilder;
use crate::builder::row_builder::RowBuilder;
use crate::model::cell::Cell;
use crate::model::row::Row;
use crate::model::table::{SectionKind, Table};
use crate::style::{Align, Color, Style};

/// Closure-DSL builder for one [`crate::model::section::Section`] (header,
/// body, or footer), passed to [`crate::builder::table_builder::TableBuilder::header`]
/// / `body` / `footer`.
pub struct SectionBuilder<'a> {
    pub(crate) table: &'a mut Table,
    pub(crate) kind: SectionKind,
}

impl<'a> SectionBuilder<'a> {
    pub(crate) fn new(table: &'a mut Table, kind: SectionKind) -> Self {
        SectionBuilder { table, kind }
    }

    /// Sets this section's style override.
    ///
    /// Merges onto whatever was already set on this builder (e.g. via
    /// [`Self::align`]), so call order doesn't matter. Pass a style marked
    /// [`Style::exact`] to hard-reset everything set so far instead.
    pub fn style(&mut self, style: Style) -> &mut Self {
        let section = self.table.section_mut(self.kind);
        section.style = if style.exact { style } else { section.style.merge(&style) };
        self
    }

    /// Sets this section's default horizontal alignment (merged into the
    /// section's style). Overrides the table-wide default for this section
    /// only (e.g. setting it on `header` does not affect `body`/`footer`),
    /// while column/row/cell-level alignment set within this section still
    /// wins over it, same as any other cascade level.
    pub fn align(&mut self, align: Align) -> &mut Self {
        let section = self.table.section_mut(self.kind);
        section.style = section.style.align(align);
        self
    }

    /// Sets this section's foreground (text) color (merged into its style).
    pub fn fg(&mut self, color: Color) -> &mut Self {
        let section = self.table.section_mut(self.kind);
        section.style = section.style.fg(color);
        self
    }

    /// Sets this section's background color (merged into its style).
    pub fn bg(&mut self, color: Color) -> &mut Self {
        let section = self.table.section_mut(self.kind);
        section.style = section.style.bg(color);
        self
    }

    /// Enables bold text for this section (merged into its style).
    pub fn bold(&mut self) -> &mut Self {
        let section = self.table.section_mut(self.kind);
        section.style = section.style.bold();
        self
    }

    /// Enables italic text for this section (merged into its style).
    pub fn italic(&mut self) -> &mut Self {
        let section = self.table.section_mut(self.kind);
        section.style = section.style.italic();
        self
    }

    /// Enables underlined text for this section (merged into its style).
    pub fn underline(&mut self) -> &mut Self {
        let section = self.table.section_mut(self.kind);
        section.style = section.style.underline();
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
    /// [`Self::row`]) wins over its zebra style. Stored directly on the
    /// [`crate::model::section::Section`] model (not just this builder), so
    /// a [`crate::theme::Theme`] can pre-populate it (e.g. `body_zebra_styles`)
    /// before this method is ever called; calling it here always replaces
    /// whatever was set before (by a theme or an earlier call).
    pub fn row_styles(&mut self, styles: impl IntoIterator<Item = Style>) -> &mut Self {
        self.table.section_mut(self.kind).zebra_styles = styles.into_iter().collect();
        self
    }

    /// Overrides style/border settings for one column (by 0-based index),
    /// scoped to this section. Returns a [`ColumnBuilder`] so settings can
    /// be chained directly, e.g. `b.column(0).align(Align::Left);`.
    pub fn column(&mut self, index: usize) -> ColumnBuilder<'_> {
        let column = self.table.section_mut(self.kind).column_mut(index);
        ColumnBuilder { column }
    }

    /// Appends a row of plain-text cells.
    pub fn row_cells<I, C>(&mut self, cells: I) -> &mut Self
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
    pub fn row(&mut self, f: impl FnOnce(&mut RowBuilder)) -> &mut Self {
        let mut row = Row::new();
        f(&mut RowBuilder { row: &mut row });
        self.push_row(row);
        self
    }

    fn push_row(&mut self, mut row: Row) {
        let zebra_styles = &self.table.section(self.kind).map(|s| s.zebra_styles.clone()).unwrap_or_default();
        if !zebra_styles.is_empty() {
            let row_index = self
                .table
                .section(self.kind)
                .map(|s| s.rows.len())
                .unwrap_or(0);
            let zebra = zebra_styles[row_index % zebra_styles.len()];
            row.style = Style::cascade([Some(&zebra), Some(&row.style)]);
        }
        self.table.push_row(self.kind, row);
    }
}
