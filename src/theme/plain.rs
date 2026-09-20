//! "Plain" theme: no borders, no styling at all — just a single space
//! between columns (falls out of the renderer's borderless-spacing rule,
//! same as [`crate::theme::github::github`]).

use crate::border::{BorderPreset, Borders};
use crate::style::Style;
use crate::theme::Theme;

/// Builds the plain [`Theme`]: no borders, no styles, columns separated by a
/// single space.
pub fn plain() -> Theme {
    Theme {
        border_preset: BorderPreset::none(),
        table_borders: Borders::NONE,
        table_style: Style::new(),
        header_style: Style::new(),
        body_zebra_styles: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Table;

    #[test]
    fn applies_no_borders_and_no_styles() {
        let table = Table::build(|t| {
            t.theme(plain());
            t.header(|h| {
                h.row_cells(["Name", "Stars"]);
            });
            t.body(|b| {
                b.row_cells(["ratatui", "12.3k"]);
            });
        });

        assert_eq!(table.borders, Some(Borders::NONE));
        assert_eq!(table.border_preset, BorderPreset::none());
    }

    #[test]
    fn renders_with_a_single_space_between_columns_and_no_border_glyphs() {
        let table = Table::build(|t| {
            t.theme(plain());
            t.header(|h| {
                h.row_cells(["Name", "Stars"]);
            });
            t.body(|b| {
                b.row_cells(["ratatui", "12.3k"]);
            });
        });

        let rendered = table.render();
        assert!(!rendered.contains('│'));
        assert!(!rendered.contains('┌'));
        assert!(!rendered.contains("\u{1b}["));
        let header_line = rendered.lines().next().unwrap();
        assert_eq!(header_line, "Name    Stars");
    }
}
