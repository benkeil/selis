//! Built-in [`Theme`] presets, applied via [`crate::builder::table_builder::TableBuilder::theme`].

pub mod classic;
pub mod github;
pub mod plain;

use crate::border::{BorderPreset, Borders};
use crate::style::Style;

/// A bundle of table-wide settings (border preset, default border sides,
/// table style, and header style) that can be applied in one call via
/// `TableBuilder::theme(...)`, instead of setting each individually.
///
/// Since `theme(...)` just sets these same fields a user could set by hand,
/// calling it before further customizing the table (e.g. `t.header(|h| {
/// h.style(..) })`) lets that customization still override the theme, same
/// as any other cascade level — call `theme(...)` first, as the least
/// specific/"default" layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub border_preset: BorderPreset,
    pub table_borders: Borders,
    pub table_style: Style,
    pub header_style: Style,
    /// Alternating (zebra-striping) styles applied to the body section's
    /// rows by position, matching [`crate::builder::section_builder::SectionBuilder::row_styles`].
    /// Empty means no zebra-striping (the default for built-in themes that
    /// don't request it). Only applied to `body` — header/footer can still
    /// set their own via `h.row_styles(...)`/`f.row_styles(...)` if desired.
    pub body_zebra_styles: Vec<Style>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_and_classic_are_distinct() {
        assert_ne!(github::github(), classic::classic());
    }
}
