//! GitHub-flavored theme: no borders at all, header cells underlined all the
//! way to the end of the cell and rendered uppercase in grey, and a single
//! space between columns (the latter falls out of the renderer's
//! borderless-spacing rule for free).

use crate::border::{BorderPreset, Borders};
use crate::style::{Color, Style};
use crate::theme::Theme;

/// Builds the GitHub-style [`Theme`].
pub fn github() -> Theme {
    Theme {
        border_preset: BorderPreset::none(),
        table_borders: Borders::NONE,
        table_style: Style::new(),
        header_style: Style::new().underline().uppercase().fg(Color::BrightBlack).bold(),
        body_zebra_styles: Vec::new(),
    }
}

/// Builds a GitHub-style [`Theme`] with subtle zebra-striping on the body
/// (every other row dimmed), otherwise identical to [`github`].
pub fn github_zebra() -> Theme {
    Theme { body_zebra_styles: vec![Style::new(), Style::new().dim()], ..github() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::table::SectionKind;
    use crate::model::Table;
    use crate::style::Case;

    #[test]
    fn applies_no_borders_and_underlines_the_header() {
        let table = Table::build(|t| {
            t.theme(github());
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
        assert_eq!(header.style.resolved_underline(), Some(true));
        assert_eq!(header.style.resolved_case(), Some(Case::Uppercase));
        assert_eq!(header.style.resolved_fg(), Some(Color::BrightBlack));
    }

    #[test]
    fn renders_without_any_border_glyphs_and_underlines_the_header_row() {
        let table = Table::build(|t| {
            t.theme(github());
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
        // The underline SGR code (4) must wrap the header line.
        let header_line = rendered.lines().next().unwrap();
        assert!(header_line.contains("\u{1b}[4m"));
        // The header content is uppercased.
        assert!(header_line.contains("NAME"));
        assert!(header_line.contains("STARS"));
    }

    #[test]
    fn github_zebra_pre_populates_the_body_sections_zebra_styles() {
        let table = Table::build(|t| {
            t.theme(github_zebra());
            t.body(|b| {
                b.row_cells(["a", "1"]);
                b.row_cells(["b", "2"]);
            });
        });

        let body = table.section(SectionKind::Body).unwrap();
        assert_eq!(body.zebra_styles.len(), 2);
        assert_eq!(body.rows[0].style.resolved_dim(), None);
        assert_eq!(body.rows[1].style.resolved_dim(), Some(true));
    }

    #[test]
    fn github_zebra_is_otherwise_identical_to_github() {
        assert_eq!(
            Theme { body_zebra_styles: Vec::new(), ..github_zebra() },
            github()
        );
    }

    #[test]
    fn a_section_can_still_override_the_themes_zebra_default() {
        let table = Table::build(|t| {
            t.theme(github_zebra());
            t.body(|b| {
                b.row_styles([Style::new().fg(Color::Green)]);
                b.row_cells(["a", "1"]);
                b.row_cells(["b", "2"]);
            });
        });

        let body = table.section(SectionKind::Body).unwrap();
        assert_eq!(body.rows[0].style.resolved_fg(), Some(Color::Green));
        assert_eq!(body.rows[1].style.resolved_fg(), Some(Color::Green));
        assert_eq!(body.rows[1].style.resolved_dim(), None);
    }
}
