//! Horizontal text alignment.

/// Horizontal alignment of a cell's content within its column width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}
