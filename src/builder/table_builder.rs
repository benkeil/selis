//! Table-level DSL: the entry point ([`crate::model::table::Table::build`])
//! plus table-wide style/border/caption settings and access to header/body/
//! footer section builders.

use crate::border::{BorderPreset, Borders};
use crate::builder::section_builder::SectionBuilder;
use crate::model::table::{SectionKind, Table};
use crate::style::{Align, Style};

/// Closure-DSL builder for a [`Table`], passed to [`Table::build`].
pub struct TableBuilder<'a> {
    pub(crate) table: &'a mut Table,
}

impl TableBuilder<'_> {
    /// Sets the table-wide style override (the least-specific level of the
    /// style cascade: table -> section -> column -> row -> cell).
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.table.style = style;
        self
    }

    /// Sets the table-wide default horizontal alignment (merged into the
    /// table's style).
    pub fn align(&mut self, align: Align) -> &mut Self {
        self.table.style = self.table.style.align(align);
        self
    }

    /// Sets the border glyph preset used to render this table's borders,
    /// e.g. [`BorderPreset::utf8_square`] or [`BorderPreset::square_double_section_separator`].
    pub fn border(&mut self, preset: BorderPreset) -> &mut Self {
        self.table.border_preset = preset;
        self
    }

    /// Sets the table-wide default border sides (the least-specific level
    /// of the border-sides cascade), matching the Mordant DSL's `tableBorders`.
    pub fn table_borders(&mut self, borders: Borders) -> &mut Self {
        self.table.borders = Some(borders);
        self
    }

    /// Sets the caption rendered above the table.
    pub fn caption_top(&mut self, caption: impl Into<String>) -> &mut Self {
        self.table.caption_top = Some(caption.into());
        self
    }

    /// Sets the caption rendered below the table.
    pub fn caption_bottom(&mut self, caption: impl Into<String>) -> &mut Self {
        self.table.caption_bottom = Some(caption.into());
        self
    }

    /// Applies a [`crate::theme::Theme`] preset (border preset, default
    /// border sides, table style, and header style) in one call. Because
    /// this simply assigns those same fields a user could set individually,
    /// call it first (before any further `header`/`body`/`footer`
    /// customization) so more specific settings still win, same as any
    /// other cascade level.
    pub fn theme(&mut self, theme: crate::theme::Theme) -> &mut Self {
        self.table.border_preset = theme.border_preset;
        self.table.borders = Some(theme.table_borders);
        self.table.style = theme.table_style;
        self.table.section_mut(SectionKind::Header).style = theme.header_style;
        self
    }

    /// Configures the header section.
    pub fn header(&mut self, f: impl FnOnce(&mut SectionBuilder)) -> &mut Self {
        f(&mut SectionBuilder::new(self.table, SectionKind::Header));
        self
    }

    /// Configures the body section.
    pub fn body(&mut self, f: impl FnOnce(&mut SectionBuilder)) -> &mut Self {
        f(&mut SectionBuilder::new(self.table, SectionKind::Body));
        self
    }

    /// Configures the footer section.
    pub fn footer(&mut self, f: impl FnOnce(&mut SectionBuilder)) -> &mut Self {
        f(&mut SectionBuilder::new(self.table, SectionKind::Footer));
        self
    }
}

impl Table {
    /// Builds a [`Table`] using the closure-based DSL, e.g.:
    ///
    /// ```
    /// use selis::Table;
    ///
    /// let table = Table::build(|t| {
    ///     t.header(|h| { h.row_cells(["Name", "Stars"]); });
    ///     t.body(|b| { b.row_cells(["ratatui", "12.3k"]); });
    /// });
    /// assert_eq!(table.column_count(), Some(2));
    /// ```
    pub fn build(f: impl FnOnce(&mut TableBuilder)) -> Table {
        let mut table = Table::new();
        f(&mut TableBuilder { table: &mut table });
        table
    }
}
