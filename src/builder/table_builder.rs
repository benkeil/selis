//! Table-level DSL: the entry point ([`crate::model::table::Table::build`])
//! plus table-wide style/border/caption settings and access to header/body/
//! footer section builders.

use crate::border::{BorderPreset, Borders};
use crate::builder::column_builder::ColumnBuilder;
use crate::builder::section_builder::SectionBuilder;
use crate::model::table::{SectionKind, Table};
use crate::style::{Align, Color, Style};

/// Closure-DSL builder for a [`Table`], passed to [`Table::build`].
pub struct TableBuilder<'a> {
    pub(crate) table: &'a mut Table,
}

impl TableBuilder<'_> {
    /// Sets the table-wide style override (the least-specific level of the
    /// style cascade: table -> section -> column -> row -> cell).
    ///
    /// Merges onto whatever was already set on this builder (e.g. via
    /// [`Self::align`]), so call order doesn't matter. Pass a style marked
    /// [`Style::exact`] to hard-reset everything set so far instead.
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.table.style = if style.exact { style } else { self.table.style.merge(&style) };
        self
    }

    /// Sets the table-wide default horizontal alignment (merged into the
    /// table's style).
    pub fn align(&mut self, align: Align) -> &mut Self {
        self.table.style = self.table.style.align(align);
        self
    }

    /// Sets the table-wide foreground (text) color (merged into its style).
    pub fn fg(&mut self, color: Color) -> &mut Self {
        self.table.style = self.table.style.fg(color);
        self
    }

    /// Sets the table-wide background color (merged into its style).
    pub fn bg(&mut self, color: Color) -> &mut Self {
        self.table.style = self.table.style.bg(color);
        self
    }

    /// Enables bold text table-wide (merged into its style).
    pub fn bold(&mut self) -> &mut Self {
        self.table.style = self.table.style.bold();
        self
    }

    /// Enables italic text table-wide (merged into its style).
    pub fn italic(&mut self) -> &mut Self {
        self.table.style = self.table.style.italic();
        self
    }

    /// Enables underlined text table-wide (merged into its style).
    pub fn underline(&mut self) -> &mut Self {
        self.table.style = self.table.style.underline();
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
        self.table.section_mut(SectionKind::Body).zebra_styles = theme.body_zebra_styles;
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

    /// Overrides style/border settings for one column (by 0-based index)
    /// across the *whole table* — header, body, and footer alike — much
    /// like selecting an entire column in a spreadsheet. Less specific than
    /// a section-scoped `column(...)` override for the same index (e.g. via
    /// [`SectionBuilder::column`]), which still wins for that section only.
    /// Returns a [`ColumnBuilder`] so settings can be chained directly, e.g.
    /// `t.column(6).align(Align::Right);`.
    pub fn column(&mut self, index: usize) -> ColumnBuilder<'_> {
        ColumnBuilder { column: self.table.column_mut(index) }
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
