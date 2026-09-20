//! The public closure-based DSL for building a [`crate::model::table::Table`],
//! e.g.:
//!
//! ```
//! use selis::prelude::*;
//!
//! let table = Table::build(|t| {
//!     t.border(BorderPreset::utf8_square());
//!     t.header(|h| {
//!         h.row(["Name", "Language"]);
//!     });
//!     t.body(|b| {
//!         b.row(["selis", "Rust"]);
//!     });
//! });
//! ```

pub mod cell_builder;
pub mod column_builder;
pub mod row_builder;
pub mod section_builder;
pub mod table_builder;

pub use cell_builder::CellBuilder;
pub use column_builder::ColumnBuilder;
pub use row_builder::RowBuilder;
pub use section_builder::SectionBuilder;
pub use table_builder::TableBuilder;
