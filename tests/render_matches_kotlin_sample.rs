//! Rebuilds the structure of the Kotlin/Mordant sample table (household
//! expenditure table with colspan, rowspan-adjacent zebra striping, per-
//! column styles, and a short footer row that gets auto-padded) using the
//! closure-based DSL, and asserts on the resulting model shape.
//!
//! This intentionally does not assert on rendered ANSI output yet (that's
//! the "renderer" phase) — it validates that the DSL can express the sample
//! 1:1 and that colspan/auto-padding/validation behave as expected.

use selis::prelude::*;
use selis::model::table::SectionKind;

#[test]
fn builds_the_kotlin_sample_table_shape() {
    let table = Table::build(|t| {
        t.border(BorderPreset::square_double_section_separator());
        t.style(Style::new().fg(Color::from_hex("#4b25b9").unwrap()));
        t.align(Align::Right);
        t.table_borders(Borders::NONE);

        t.header(|h| {
            h.style(Style::new().fg(Color::BrightRed).bold());

            h.row_with(|r| {
                r.cell_borders(Borders::NONE);
                r.cells(["", "", "", ""]);
                r.cell("Percent Change", |c| {
                    c.colspan(2);
                    c.align(Align::Center);
                });
            });

            h.row_with(|r| {
                r.cell_borders(Borders::BOTTOM);
                r.cells(["", "2020", "2021", "2022", "2020-21", "2021-22"]);
            });
        });

        t.body(|b| {
            b.style(Style::new().fg(Color::Green));
            b.cell_borders(Borders::TOP_BOTTOM);

            b.column(0, |c| {
                c.align(Align::Left);
                c.cell_borders(Borders::ALL);
                c.style(Style::new().fg(Color::BrightBlue));
            });
            b.column(4, |c| {
                c.cell_borders(Borders::LEFT_BOTTOM);
                c.style(Style::new().fg(Color::BrightBlue));
            });
            b.column(5, |c| {
                c.style(Style::new().fg(Color::BrightBlue));
            });

            b.row_styles([Style::new(), Style::new().dim()]);

            b.row(["Average income before taxes", "$84,352", "$87,432", "$94,003", "3.7", "7.5"]);
            b.row(["Average annual expenditures", "$61,332", "$66,928", "$72,967", "9.1", "9.0"]);
            b.row(["  Food", "7,310", "8,289", "9,343", "13.4", "12.7"]);
            b.row(["  Housing", "21,417", "22,624", "24,298", "5.6", "7.4"]);
            b.row(["  Apparel and services", "1,434", "1,754", "1,945", "22.3", "10.9"]);
            b.row(["  Transportation", "9,826", "10,961", "12,295", "11.6", "12.2"]);
            b.row(["  Healthcare", "5,177", "5,452", "5,850", "5.3", "7.3"]);
            b.row(["  Entertainment", "2,909", "3,568", "3,458", "22.7", "-3.1"]);
            b.row(["  Education", "1,271", "1,226", "1,335", "-3.5", "8.9"]);
        });

        t.footer(|f| {
            f.style(Style::new().italic());
            f.row_with(|r| {
                r.cells(["Remaining income", "$23,020", "$20,504", "$21,036"]);
            });
        });

        t.caption_bottom(Style::new().dim().render("via U.S. Bureau of Labor Statistics"));
    });

    // Column count is established by the first header row (4 plain cells +
    // 1 colspan-2 cell = 6 columns).
    assert_eq!(table.column_count(), Some(6));

    let header = table.header.as_ref().unwrap();
    assert_eq!(header.rows.len(), 2);
    assert_eq!(header.rows[0].cells.len(), 5); // 4 plain + 1 colspan(2) cell
    assert_eq!(header.rows[0].cells[4].colspan, 2);
    assert_eq!(header.rows[0].cells[4].content, "Percent Change");
    assert_eq!(header.rows[1].cells.len(), 6);

    assert_eq!(table.body.rows.len(), 9);
    assert_eq!(table.body.columns.len(), 3); // columns 0, 4, 5 have overrides
    assert_eq!(table.body.columns[&0].style.resolved_align(), Align::Left);

    // The footer row only declared 4 cells; auto-padding happens during
    // grid resolution (rowspan-aware), not on the raw model.
    let footer = table.footer.as_ref().unwrap();
    assert_eq!(footer.rows[0].cells.len(), 4);
    let footer_grid = selis::layout::grid::resolve(footer, table.column_count().unwrap());
    for col in 4..6 {
        assert!(matches!(
            footer_grid.slots[0][col],
            selis::layout::grid::GridSlot::Origin { source_cell: None, .. }
        ));
    }

    assert!(table.caption_bottom.is_some());
    assert_eq!(table.borders, Some(Borders::NONE));
}

#[test]
#[should_panic(expected = "row has 7 cells (considering colspan) but the table only has 6 columns")]
fn a_row_longer_than_the_established_column_count_panics() {
    Table::build(|t| {
        t.header(|h| {
            h.row(["a", "b", "c", "d", "e", "f"]);
        });
        t.footer(|f| {
            f.row(["a", "b", "c", "d", "e", "f", "g"]);
        });
    });
}

#[test]
fn zebra_row_styles_alternate_and_explicit_style_wins() {
    let table = Table::build(|t| {
        t.body(|b| {
            b.row_styles([Style::new().fg(Color::Green), Style::new().dim()]);
            b.row(["a"]);
            b.row(["b"]);
            b.row_with(|r| {
                r.cells(["c"]);
                r.style(Style::new().fg(Color::Red));
            });
        });
    });

    assert_eq!(table.body.rows[0].style.resolved_align(), Align::Left); // sanity: default align
    assert_eq!(table.body.rows[0].style, Style::new().fg(Color::Green));
    assert_eq!(table.body.rows[1].style, Style::new().dim());
    // row 2's explicit fg (Red) wins over its zebra style (Green, since 2 % 2 == 0).
    let resolved = Style::cascade([Some(&Style::new().fg(Color::Green)), Some(&table.body.rows[2].style)]);
    assert_eq!(resolved.resolved_fg(), Some(Color::Red));
}

// Ensure `SectionKind` re-export path used above stays valid.
#[allow(dead_code)]
fn _use_section_kind(_: SectionKind) {}
