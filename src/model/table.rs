//! The top-level [`Table`]: sections (header/body/footer), table-wide style/
//! border settings, and the row-count validation/auto-padding described in
//! the crate's design notes.

use std::collections::BTreeMap;

use crate::border::{BorderPreset, Borders};
use crate::model::column::Column;
use crate::model::row::Row;
use crate::model::section::Section;
use crate::style::Style;

/// Which section a row belongs to, used by [`Table::push_row`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionKind {
    Header,
    Body,
    Footer,
}

/// A complete table: header/body/footer sections, table-wide style and
/// border settings, and an optional caption.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Table {
    pub style: Style,
    pub borders: Option<Borders>,
    pub border_preset: BorderPreset,
    pub header: Option<Section>,
    pub body: Section,
    pub footer: Option<Section>,
    pub caption_top: Option<String>,
    pub caption_bottom: Option<String>,
    /// Table-wide per-column overrides, keyed by (0-based) column index,
    /// applying across header/body/footer alike (an "Excel-style" whole-
    /// column selection). Less specific than a section-scoped
    /// [`Section::columns`] override for the same index, which still wins
    /// for that section.
    pub columns: BTreeMap<usize, Column>,
    /// The number of columns in the table. Fixed by whichever row is pushed
    /// first (via [`Table::push_row`]); every row pushed afterwards is
    /// validated (or auto-padded) against it. `None` until the first row is
    /// pushed.
    column_count: Option<usize>,
}

impl Table {
    /// Creates an empty table (an empty body, no header/footer yet).
    pub fn new() -> Self {
        Self::default()
    }

    /// The table's column count, if established yet (i.e. if at least one
    /// row has been pushed).
    pub fn column_count(&self) -> Option<usize> {
        self.column_count
    }

    /// Sets this table's style override (the least-specific level of the
    /// style cascade).
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets this table's default border sides (the least-specific level of
    /// the border-sides cascade), matching the Mordant DSL's `tableBorders`.
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = Some(borders);
        self
    }

    /// Sets the border glyph preset (character set) used to render this
    /// table's borders.
    pub fn border_preset(mut self, preset: BorderPreset) -> Self {
        self.border_preset = preset;
        self
    }

    /// Sets the caption rendered above the table.
    pub fn caption_top(mut self, caption: impl Into<String>) -> Self {
        self.caption_top = Some(caption.into());
        self
    }

    /// Sets the caption rendered below the table.
    pub fn caption_bottom(mut self, caption: impl Into<String>) -> Self {
        self.caption_bottom = Some(caption.into());
        self
    }

    /// Returns a reference to the requested section, if present (header and
    /// footer are optional; body always exists).
    pub fn section(&self, kind: SectionKind) -> Option<&Section> {
        match kind {
            SectionKind::Header => self.header.as_ref(),
            SectionKind::Body => Some(&self.body),
            SectionKind::Footer => self.footer.as_ref(),
        }
    }

    /// Returns a mutable reference to the requested section, creating it
    /// (header/footer only) if it doesn't exist yet.
    pub fn section_mut(&mut self, kind: SectionKind) -> &mut Section {
        match kind {
            SectionKind::Header => self.header.get_or_insert_with(Section::new),
            SectionKind::Body => &mut self.body,
            SectionKind::Footer => self.footer.get_or_insert_with(Section::new),
        }
    }

    /// Returns a mutable reference to the table-wide override for
    /// `column_index`, creating a default one if it doesn't exist yet.
    pub fn column_mut(&mut self, column_index: usize) -> &mut Column {
        self.columns.entry(column_index).or_default()
    }

    /// Appends `row` to the given section, after validating/fixing it up
    /// against the table's column count:
    ///
    /// - If this is the very first row pushed to the table, it establishes
    ///   the table's column count (the sum of its cells' `colspan`s).
    /// - If the row has *fewer* cells (by total colspan) than the column
    ///   count, it is auto-padded with empty cells.
    /// - If the row has *more* cells (by total colspan) than the column
    ///   count, this panics with a clear message — this is a programming
    ///   error in the table definition, not a recoverable runtime condition.
    pub fn push_row(&mut self, kind: SectionKind, mut row: Row) {
        self.fit_row_to_column_count(&mut row);
        self.section_mut(kind).rows.push(row);
    }

    /// Renders this table to a final ANSI string (no trailing newline),
    /// resolving the full style/border cascade and colspan/rowspan layout.
    pub fn render(&self) -> String {
        crate::render::render(self)
    }

    fn fit_row_to_column_count(&mut self, row: &mut Row) {
        let row_width = row.column_span_width();
        let column_count = *self.column_count.get_or_insert(row_width);

        if row_width > column_count {
            panic!(
                "row has {row_width} cells (considering colspan) but the table only has \
                 {column_count} columns: {:?}",
                row.cells.iter().map(|c| c.content.as_str()).collect::<Vec<_>>()
            );
        }

        // Rows with fewer cells than the column count are auto-padded at
        // grid-resolution time (see `crate::layout::grid::resolve`), since
        // whether trailing columns are actually free depends on rowspans
        // carried over from earlier rows in the same section — information
        // this method doesn't have (it only sees one row at a time). This
        // check only catches the simple, rowspan-independent "too many
        // cells" case early, right at the `row(...)` call site; the full,
        // rowspan-aware check happens in `grid::resolve`.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::cell::Cell;

    #[test]
    fn first_row_establishes_column_count() {
        let mut table = Table::new();
        table.push_row(SectionKind::Header, Row::from_cells(["a", "b", "c"]));
        assert_eq!(table.column_count(), Some(3));
    }

    #[test]
    fn colspan_counts_toward_column_width() {
        let mut table = Table::new();
        let mut row = Row::new();
        row.push(Cell::new("wide").colspan(2));
        row.push(Cell::new("normal"));
        table.push_row(SectionKind::Header, row);
        assert_eq!(table.column_count(), Some(3));
    }

    #[test]
    fn shorter_row_no_longer_gets_padded_at_push_time() {
        // Padding now happens during grid resolution (rowspan-aware); at
        // the model level, a short row simply stays short.
        let mut table = Table::new();
        table.push_row(SectionKind::Header, Row::from_cells(["a", "b", "c", "d", "e", "f"]));
        table.push_row(
            SectionKind::Footer,
            Row::from_cells(["Remaining income", "$23,020", "$20,504", "$21,036"]),
        );

        let footer_row = &table.footer.as_ref().unwrap().rows[0];
        assert_eq!(footer_row.cells.len(), 4);
    }

    #[test]
    #[should_panic(expected = "row has 7 cells (considering colspan) but the table only has 6 columns")]
    fn longer_row_panics() {
        let mut table = Table::new();
        table.push_row(SectionKind::Header, Row::from_cells(["a", "b", "c", "d", "e", "f"]));
        table.push_row(SectionKind::Body, Row::from_cells(["a", "b", "c", "d", "e", "f", "g"]));
    }

    #[test]
    fn rows_pushed_into_different_sections_share_the_same_column_count() {
        let mut table = Table::new();
        table.push_row(SectionKind::Header, Row::from_cells(["a", "b"]));
        table.push_row(SectionKind::Body, Row::from_cells(["c", "d"]));
        assert_eq!(table.body.rows[0].cells.len(), 2);
    }
}
