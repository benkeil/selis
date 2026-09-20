//! Layout: colspan/rowspan grid resolution, column width computation, and
//! border-grid junction resolution — everything needed to turn a [`Table`]
//! model into concrete geometry, before [`crate::render`] turns that into text.
//!
//! [`Table`]: crate::model::table::Table

pub mod border_grid;
pub mod grid;
pub mod width;

pub use border_grid::BorderGrid;
pub use grid::{Grid, GridSlot};
pub use width::{compute_column_widths, SectionGrid};
