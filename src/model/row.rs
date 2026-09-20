//! A row of cells, plus row-level style/border overrides.

use crate::border::Borders;
use crate::model::cell::Cell;
use crate::style::Style;

/// One row of a [`crate::model::section::Section`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub style: Style,
    pub borders: Option<Borders>,
}

impl Row {
    /// Creates an empty row (no cells yet).
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a row from a list of plain-text cells.
    pub fn from_cells<I, C>(cells: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Cell>,
    {
        Row {
            cells: cells.into_iter().map(Into::into).collect(),
            ..Self::default()
        }
    }

    /// Appends a cell.
    pub fn push(&mut self, cell: impl Into<Cell>) -> &mut Self {
        self.cells.push(cell.into());
        self
    }

    /// Sets this row's style override.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets this row's border-side override.
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = Some(borders);
        self
    }

    /// The total number of columns this row occupies, i.e. the sum of every
    /// cell's `colspan`.
    pub fn column_span_width(&self) -> usize {
        self.cells.iter().map(|c| c.colspan).sum()
    }
}
