//! Locks down the exact rendered output for a simple, colorless table with
//! the default (UTF-8 square) border preset, so future refactors don't
//! silently change the renderer's output.

use selis::prelude::*;

#[test]
fn renders_a_simple_bordered_table_exactly() {
    let table = Table::build(|t| {
        t.border(BorderPreset::utf8_square());
        t.header(|h| {
            h.row(["Name", "Stars", "Language"]);
        });
        t.body(|b| {
            b.row(["ratatui", "12.3k", "Rust"]);
            b.row(["mordant", "1.2k", "Kotlin"]);
        });
    });

    let expected = "\
┌───────┬─────┬────────┐
│Name   │Stars│Language│
├───────┼─────┼────────┤
│ratatui│12.3k│Rust    │
├───────┼─────┼────────┤
│mordant│1.2k │Kotlin  │
└───────┴─────┴────────┘";

    assert_eq!(table.render(), expected);
}

#[test]
fn renders_colspan_and_rowspan_correctly() {
    let table = Table::build(|t| {
        t.border(BorderPreset::utf8_square());
        t.body(|b| {
            b.row_with(|r| {
                r.cell("Kategorie", |c| {
                    c.rowspan(3);
                });
                r.cells(["Essen", "9343"]);
            });
            b.row(["Wohnen", "24298"]);
            b.row(["Transport", "12295"]);
        });
    });

    let expected = "\
┌─────────┬─────────┬─────┐
│Kategorie│Essen    │9343 │
│         ├─────────┼─────┤
│         │Wohnen   │24298│
│         ├─────────┼─────┤
│         │Transport│12295│
└─────────┴─────────┴─────┘";

    assert_eq!(table.render(), expected);
}

#[test]
fn no_borders_uses_single_space_separators_and_no_outer_edges() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.header(|h| {
            h.row(["Name", "Stars"]);
        });
        t.body(|b| {
            b.row(["ratatui", "12.3k"]);
        });
    });

    let expected = "\
Name    Stars
ratatui 12.3k";

    assert_eq!(table.render(), expected);
}
