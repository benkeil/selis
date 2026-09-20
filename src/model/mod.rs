//! Data model: `Table`, `Section`, `Row`, `Cell`, `Column`.

pub mod cell;
pub mod column;
pub mod max_width;
pub mod row;
pub mod section;
pub mod table;

pub use cell::Cell;
pub use column::Column;
pub use max_width::MaxWidth;
pub use row::Row;
pub use section::Section;
pub use table::{SectionKind, Table};
