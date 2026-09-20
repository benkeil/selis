//! "Rounded" theme: rounded-corner Unicode borders with a bold, cyan header
//! — a modern-looking default that still draws a full grid, unlike the
//! borderless [`crate::theme::github`]/[`crate::theme::plain`] themes.

use crate::border::{BorderPreset, Borders};
use crate::style::{Color, Style};
use crate::theme::Theme;

/// Builds the rounded [`Theme`] (rounded-corner borders, bold cyan header).
pub fn rounded() -> Theme {
    Theme {
        border_preset: BorderPreset::utf8_rounded(),
        table_borders: Borders::ALL,
        table_style: Style::new(),
        header_style: Style::new().bold().fg(Color::Cyan),
        body_zebra_styles: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::table::SectionKind;
    use crate::model::Table;

    #[test]
    fn applies_rounded_borders_and_a_bold_cyan_header() {
        let table = Table::build(|t| {
            t.theme(rounded());
            t.header(|h| {
                h.row_cells(["Name", "Stars"]);
            });
            t.body(|b| {
                b.row_cells(["ratatui", "12.3k"]);
            });
        });

        assert_eq!(table.borders, Some(Borders::ALL));
        assert_eq!(table.border_preset, BorderPreset::utf8_rounded());
        let header = table.section(SectionKind::Header).unwrap();
        assert_eq!(header.style.resolved_bold(), Some(true));
        assert_eq!(header.style.resolved_fg(), Some(Color::Cyan));
    }

    #[test]
    fn renders_with_rounded_corner_glyphs() {
        let table = Table::build(|t| {
            t.theme(rounded());
            t.header(|h| {
                h.row_cells(["Name", "Stars"]);
            });
            t.body(|b| {
                b.row_cells(["ratatui", "12.3k"]);
            });
        });

        let rendered = table.render();
        assert!(rendered.contains('╭'));
        assert!(rendered.contains('╯'));
    }
}
