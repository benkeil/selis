//! "Minimal zebra" theme: no borders and no header styling — just subtle
//! zebra-striping (every other row dimmed) to keep dense data tables
//! readable, matching [`crate::theme::plain`] except for the striping.

use crate::border::{BorderPreset, Borders};
use crate::style::Style;
use crate::theme::Theme;

/// Builds the minimal-zebra [`Theme`]: no borders, no header styling, body
/// rows alternate between unstyled and dimmed.
pub fn minimal_zebra() -> Theme {
    Theme {
        border_preset: BorderPreset::none(),
        table_borders: Borders::NONE,
        table_style: Style::new(),
        header_style: Style::new(),
        body_zebra_styles: vec![Style::new(), Style::new().dim()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::table::SectionKind;
    use crate::model::Table;

    #[test]
    fn applies_no_borders_and_no_header_style() {
        let table = Table::build(|t| {
            t.theme(minimal_zebra());
            t.header(|h| {
                h.row_cells(["Name", "Stars"]);
            });
            t.body(|b| {
                b.row_cells(["ratatui", "12.3k"]);
            });
        });

        assert_eq!(table.borders, Some(Borders::NONE));
        assert_eq!(table.border_preset, BorderPreset::none());
        let header = table.section(SectionKind::Header).unwrap();
        assert_eq!(header.style.resolved_bold(), None);
    }

    #[test]
    fn body_rows_alternate_dim_by_position() {
        let table = Table::build(|t| {
            t.theme(minimal_zebra());
            t.body(|b| {
                b.row_cells(["a", "1"]);
                b.row_cells(["b", "2"]);
            });
        });

        let body = table.section(SectionKind::Body).unwrap();
        assert_eq!(body.rows[0].style.resolved_dim(), None);
        assert_eq!(body.rows[1].style.resolved_dim(), Some(true));
    }

    #[test]
    fn a_section_can_still_override_the_themes_zebra_default() {
        use crate::style::Color;

        let table = Table::build(|t| {
            t.theme(minimal_zebra());
            t.body(|b| {
                b.row_styles([Style::new().fg(Color::Green)]);
                b.row_cells(["a", "1"]);
                b.row_cells(["b", "2"]);
            });
        });

        let body = table.section(SectionKind::Body).unwrap();
        assert_eq!(body.rows[0].style.resolved_fg(), Some(Color::Green));
        assert_eq!(body.rows[1].style.resolved_dim(), None);
    }
}
