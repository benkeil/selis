//! Locks down the exact rendered output for a simple, colorless table with
//! the default (UTF-8 square) border preset, so future refactors don't
//! silently change the renderer's output.

use selis::prelude::*;

/// Strips ANSI SGR escape sequences (`\x1b[...m`), leaving only the plain
/// text/whitespace layout — used by tests that only care about alignment,
/// not about the exact color/bold codes produced.
fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for next in chars.by_ref() {
                if next == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[test]
fn renders_a_simple_bordered_table_exactly() {
    let table = Table::build(|t| {
        t.border(BorderPreset::utf8_square());
        t.header(|h| {
            h.row_cells(["Name", "Stars", "Language"]);
        });
        t.body(|b| {
            b.row_cells(["ratatui", "12.3k", "Rust"]);
            b.row_cells(["mordant", "1.2k", "Kotlin"]);
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
            b.row(|r| {
                r.cell("Kategorie").rowspan(3);
                r.cells(["Essen", "9343"]);
            });
            b.row_cells(["Wohnen", "24298"]);
            b.row_cells(["Transport", "12295"]);
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
            h.row_cells(["Name", "Stars"]);
        });
        t.body(|b| {
            b.row_cells(["ratatui", "12.3k"]);
        });
    });

    let expected = "\
Name    Stars
ratatui 12.3k";

    assert_eq!(table.render(), expected);
}

#[test]
fn section_level_align_overrides_table_default_only_within_that_section() {
    let table = Table::build(|t| {
        t.align(Align::Right);
        t.table_borders(Borders::NONE);

        t.header(|h| {
            h.align(Align::Left);
            h.row_cells(["Name"]);
        });

        t.body(|b| {
            b.row_cells(["ratatui"]);
        });
    });

    // Header's own align (Left) overrides the table default (Right), but
    // only for the header section; body sets nothing itself so it still
    // falls back to the table default. Verified via the rendered output:
    // the header cell is left-aligned while the body cell is right-aligned.
    let rendered = table.render();
    let mut lines = rendered.lines();
    assert_eq!(lines.next().unwrap(), "Name   ");
    assert_eq!(lines.next().unwrap(), "ratatui");
}

#[test]
fn body_level_align_is_a_default_that_an_individual_cell_can_still_override() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);

        t.body(|b| {
            b.align(Align::Right);
            b.row(|r| {
                r.cell("a").align(Align::Left);
            });
            b.row_cells(["bb"]);
        });
    });

    let expected = "\
a 
bb";
    assert_eq!(table.render(), expected);
}


#[test]
fn style_call_after_align_does_not_wipe_the_align_regardless_of_builder_level() {
    // Reproduces the exact demo.rs pattern: a column builder calls `.align()`
    // and later `.style(...)` on the same builder. `.style()` must merge
    // onto the existing style, not replace it, so the align set earlier
    // survives.
    let table = Table::build(|t| {
        t.align(Align::Right);
        t.table_borders(Borders::NONE);

        t.body(|b| {
            b.column(0).align(Align::Left).style(Style::new().bold());
            b.row_cells(["a very long name", "x"]);
            b.row_cells(["short", "y"]);
        });
    });

    let expected = "\
a very long name x
short            y";
    assert_eq!(strip_ansi(&table.render()), expected);
}

#[test]
fn style_set_before_align_on_the_same_builder_still_keeps_both() {
    // Same fix, opposite call order: `.style()` first, `.align()` after —
    // both should still combine correctly (this already worked before the
    // fix, since it's the reverse case, but locks in the order-independence
    // going forward).
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(0).style(Style::new().bold()).align(Align::Right);
            b.row_cells(["a"]);
            b.row_cells(["bb"]);
        });
    });

    let expected = " a\nbb";
    assert_eq!(strip_ansi(&table.render()), expected);
}

#[test]
fn exact_style_still_hard_resets_a_builder_regardless_of_prior_calls() {
    // The `.exact()` escape hatch must still fully discard whatever was
    // previously set on the same builder, even after the merge fix.
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(0).align(Align::Right).style(Style::new().align(Align::Left).exact());
            b.row_cells(["a"]);
            b.row_cells(["bb"]);
        });
    });

    let expected = "\
a 
bb";
    assert_eq!(table.render(), expected);
}

#[test]
fn table_wide_column_applies_across_header_body_and_footer() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.column(0).align(Align::Right);

        t.header(|h| {
            h.row_cells(["Name"]);
        });
        t.body(|b| {
            b.row_cells(["a"]);
        });
        t.footer(|f| {
            f.row_cells(["ft"]);
        });
    });

    let expected = "\
Name
   a
  ft";
    assert_eq!(table.render(), expected);
}

#[test]
fn section_scoped_column_override_still_wins_over_a_table_wide_column_override() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.column(0).align(Align::Right);

        t.header(|h| {
            h.row_cells(["Name"]);
        });
        t.body(|b| {
            // Body overrides column 0 to Left, narrower than the table-wide
            // Right default, only for the body section.
            b.column(0).align(Align::Left);
            b.row_cells(["a"]);
        });
        t.footer(|f| {
            f.row_cells(["ft"]);
        });
    });

    let expected = "\
Name
a   
  ft";
    assert_eq!(table.render(), expected);
}

#[test]
fn max_width_truncates_overflowing_content_with_an_ellipsis() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(0).max_width(10);
            b.row_cells(["a very long piece of text", "x"]);
            b.row_cells(["short", "y"]);
        });
    });

    let expected = "\
a very ... x
short      y";
    assert_eq!(table.render(), expected);
}

#[test]
fn max_width_wider_than_content_has_no_effect() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(0).max_width(100);
            b.row_cells(["short"]);
        });
    });

    assert_eq!(table.render(), "short");
}

#[test]
fn table_wide_max_width_applies_across_header_body_and_footer() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.column(0).max_width(7);

        t.header(|h| {
            h.row_cells(["A very long header"]);
        });
        t.body(|b| {
            b.row_cells(["A very long body cell"]);
        });
        t.footer(|f| {
            f.row_cells(["A very long footer"]);
        });
    });

    let expected = "\
A ve...
A ve...
A ve...";
    assert_eq!(table.render(), expected);
}

#[test]
fn cell_truncate_replaces_just_that_cells_content_immediately() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.row(|r| {
                r.cell("a very long piece of text").truncate(10);
                r.cell("x");
            });
            b.row_cells(["short row here", "y"]);
        });
    });

    // `.truncate(10)` mutates the first cell's content immediately, before
    // layout ever runs — so the column is naturally sized around the two
    // *actual* (already-truncated / untouched) contents: "a very ..." (10)
    // and "short row here" (14). The shorter, truncated cell is then padded
    // out to that natural column width like any other cell.
    let expected = "\
a very ...     x
short row here y";
    assert_eq!(table.render(), expected);
}

#[test]
fn cell_truncate_with_uses_a_custom_ellipsis() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.row(|r| {
                r.cell("a very long piece of text").truncate_with(8, "…");
            });
        });
    });

    assert_eq!(table.render(), "a very …");
}

#[test]
fn cell_fg_shortcut_matches_explicit_style_call() {
    // `.fg(color)` must produce byte-for-byte identical output to the
    // longer-form `.style(Style::new().fg(color))`.
    let via_shortcut = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.row(|r| {
                r.cell("x").fg(Color::Green);
            });
        });
    })
    .render();

    let via_style = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.row(|r| {
                r.cell("x").style(Style::new().fg(Color::Green));
            });
        });
    })
    .render();

    assert_eq!(via_shortcut, via_style);
}

#[test]
fn style_shortcuts_are_available_on_every_builder_level() {
    // Table/section/column/row/cell all expose the same fg/bg/bold/italic/
    // underline shortcuts, matching the pre-existing `align()` shortcut.
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.fg(Color::Red).bg(Color::Black).bold().italic().underline();

        t.body(|b| {
            b.fg(Color::Red).bg(Color::Black).bold().italic().underline();
            b.column(0).fg(Color::Red).bg(Color::Black).bold().italic().underline();
            b.row(|r| {
                r.fg(Color::Red).bg(Color::Black).bold().italic().underline();
                r.cell("x").fg(Color::Red).bg(Color::Black).bold().italic().underline();
            });
        });
    });

    // Just verifying it builds and renders without panicking; the actual
    // fg/bg/bold/italic/underline cascade resolution is already covered by
    // `Style::merge`/`Style::cascade` unit tests.
    assert!(!table.render().is_empty());
}

#[test]
fn show_header_defaults_to_true_and_renders_as_before() {
    let table = Table::build(|t| {
        t.border(BorderPreset::utf8_square());
        t.header(|h| {
            h.row_cells(["Name", "Stars"]);
        });
        t.body(|b| {
            b.row_cells(["ratatui", "12.3k"]);
        });
    });

    let expected = "\
┌───────┬─────┐
│Name   │Stars│
├───────┼─────┤
│ratatui│12.3k│
└───────┴─────┘";

    assert_eq!(table.render(), expected);
}

#[test]
fn show_header_false_suppresses_header_rows_and_separator() {
    let table = Table::build(|t| {
        t.border(BorderPreset::utf8_square());
        t.header(|h| {
            h.row_cells(["Name", "Stars"]);
        });
        t.body(|b| {
            b.row_cells(["ratatui", "12.3k"]);
        });
        t.show_header(false);
    });

    let expected = "\
┌───────┬─────┐
│ratatui│12.3k│
└───────┴─────┘";

    assert_eq!(table.render(), expected);
}

#[test]
fn show_footer_false_suppresses_footer_rows_and_separator() {
    let table = Table::build(|t| {
        t.border(BorderPreset::utf8_square());
        t.body(|b| {
            b.row_cells(["ratatui", "12.3k"]);
        });
        t.footer(|f| {
            f.row_cells(["Total", "12.3k"]);
        });
        t.show_footer(false);
    });

    let expected = "\
┌───────┬─────┐
│ratatui│12.3k│
└───────┴─────┘";

    assert_eq!(table.render(), expected);
}

#[test]
fn show_header_false_does_not_affect_column_count_or_stored_rows() {
    let table = Table::build(|t| {
        t.header(|h| {
            h.row_cells(["Name", "Stars"]);
        });
        t.body(|b| {
            b.row_cells(["ratatui", "12.3k"]);
        });
        t.show_header(false);
    });

    assert_eq!(table.column_count(), Some(2));
    assert_eq!(table.header.as_ref().unwrap().rows[0].cells.len(), 2);
}

#[test]
fn style_shortcuts_merge_with_previously_set_fields_on_the_same_builder() {
    // A shortcut like `.fg(...)` must merge onto the builder's style just
    // like `.align()` does, so a preceding `.align()` call survives.
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(0).align(Align::Left).fg(Color::Green);
            b.row_cells(["a very long name", "x"]);
            b.row_cells(["short", "y"]);
        });
    });

    let expected = "\
a very long name x
short            y";
    assert_eq!(strip_ansi(&table.render()), expected);
}
