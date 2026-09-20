//! "Classic" theme: close to the Kotlin Mordant sample, using the
//! double-line section-separator border preset and a bold header.

use crate::border::{BorderPreset, Borders};
use crate::style::Style;
use crate::theme::Theme;

/// Builds the classic [`Theme`] (square borders, double lines separating
/// header/body/footer, bold header).
pub fn classic() -> Theme {
    Theme {
        border_preset: BorderPreset::square_double_section_separator(),
        table_borders: Borders::ALL,
        table_style: Style::new(),
        header_style: Style::new().bold(),
        body_zebra_styles: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::table::SectionKind;
    use crate::model::Table;

    #[test]
    fn applies_double_section_separator_borders_and_bold_header() {
        let table = Table::build(|t| {
            t.theme(classic());
            t.header(|h| {
                h.row_cells(["Name", "Stars"]);
            });
            t.body(|b| {
                b.row_cells(["ratatui", "12.3k"]);
            });
        });

        assert_eq!(table.borders, Some(Borders::ALL));
        assert_eq!(table.border_preset, BorderPreset::square_double_section_separator());
        let header = table.section(SectionKind::Header).unwrap();
        assert_eq!(header.style.resolved_bold(), Some(true));
    }
}
