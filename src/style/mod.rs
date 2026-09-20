//! Cascading style types: `Color`, `Align`, `Style`.

pub mod align;
pub mod case;
pub mod color;
#[allow(clippy::module_inception)]
pub mod style;

pub use align::Align;
pub use case::Case;
pub use color::Color;
pub use style::Style;
