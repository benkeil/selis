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
a very lo… x
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
A very…
A very…
A very…";
    assert_eq!(table.render(), expected);
}

#[test]
fn min_width_pads_a_column_narrower_than_its_content() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(0).min_width(10);
            b.row_cells(["a", "x"]);
        });
    });

    assert_eq!(table.render(), "a          x");
}

#[test]
fn min_width_has_no_effect_when_content_is_already_wider() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(0).min_width(3);
            b.row_cells(["a very long value", "x"]);
        });
    });

    assert_eq!(table.render(), "a very long value x");
}

#[test]
fn fixed_width_pads_or_truncates_to_exactly_that_width() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(0).width(6);
            b.row_cells(["a", "x"]);
            b.row_cells(["a very long value", "y"]);
        });
    });

    let expected = "\
a      x
a ver… y";
    assert_eq!(table.render(), expected);
}

#[test]
#[should_panic(expected = "column 0 has conflicting `max_width` overrides across sections")]
fn conflicting_section_scoped_width_overrides_panic() {
    let table = Table::build(|t| {
        t.header(|h| {
            h.column(0).max_width(40);
            h.row_cells(["Name"]);
        });
        t.body(|b| {
            b.column(0).max_width(50);
            b.row_cells(["a"]);
        });
    });

    table.render();
}

#[test]
fn same_value_from_two_sections_is_not_a_conflict() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.header(|h| {
            h.column(0).max_width(10);
            h.row_cells(["a very long header"]);
        });
        t.body(|b| {
            b.column(0).max_width(10);
            b.row_cells(["a very long body cell"]);
        });
    });

    let expected = "\
a very lo…
a very lo…";
    assert_eq!(table.render(), expected);
}

#[test]
fn colspanning_cell_gets_the_combined_width_of_every_column_it_spans() {
    // Column 0 is capped at max_width(1), but the colspan(2) cell still
    // spans both columns: its available room is column 0's (clamped) width
    // *plus* column 1's, not just column 0's 1 character alone — otherwise
    // it would truncate down to a single character instead of six.
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(0).max_width(1);
            b.row(|r| {
                r.cell("abcdefgh").colspan(2);
            });
            b.row_cells(["x", "y"]);
        });
    });

    let expected = "\
abcde…
x y   ";
    assert_eq!(table.render(), expected);
}

#[test]
fn cap_to_width_shrinks_the_widest_column_first_until_it_fits() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.cap_to_width(9);
        t.body(|b| {
            b.row_cells(["a very long description", "id"]);
        });
    });

    let expected = "\
a ver… id";
    assert_eq!(table.render(), expected);
}

#[test]
fn cap_to_width_stops_shrinking_at_min_width() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.cap_to_width(1); // impossible to reach
        t.body(|b| {
            b.column(0).min_width(4);
            b.row_cells(["a very long description", "id"]);
        });
    });

    // Column 0 stops at its min_width (4); column 1 stops at the default
    // floor of 1. Over budget stays over budget rather than erroring.
    let expected = "\
a v… i";
    assert_eq!(table.render(), expected);
}

#[test]
fn flex_column_grows_to_fill_whatever_width_cap_to_width_leaves_over() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.cap_to_width(10);
        t.body(|b| {
            b.column(1).flex();
            b.row_cells(["id", "x"]);
        });
    });

    let expected = "\
id x      ";
    assert_eq!(table.render(), expected);
}

#[test]
fn flex_column_shrinks_and_truncates_when_there_is_no_slack() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.cap_to_width(6);
        t.body(|b| {
            b.column(1).flex();
            b.row_cells(["id", "a very long description"]);
        });
    });

    let expected = "\
id a …";
    assert_eq!(table.render(), expected);
}

#[test]
fn flex_column_growth_is_still_capped_by_its_own_max_width() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.cap_to_width(20);
        t.body(|b| {
            b.column(1).flex().max_width(5);
            b.row_cells(["id", "x"]);
        });
    });

    let expected = "\
id x    ";
    assert_eq!(table.render(), expected);
}

#[test]
#[should_panic(expected = "only one column may be marked `flex`")]
fn more_than_one_flex_column_panics() {
    let table = Table::build(|t| {
        t.cap_to_width(20);
        t.body(|b| {
            b.column(0).flex();
            b.column(1).flex();
            b.row_cells(["a", "b"]);
        });
    });
    table.render();
}

#[test]
fn flex_without_cap_to_width_is_a_no_op() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.body(|b| {
            b.column(1).flex();
            b.row_cells(["id", "x"]);
        });
    });

    let expected = "\
id x";
    assert_eq!(table.render(), expected);
}

#[test]
fn cell_truncate_applies_before_natural_column_width_is_computed() {
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

    // `.truncate(10)` is resolved before natural column widths are
    // computed — so the column is sized around the two *already-truncated*
    // contents: "a very lo…" (10) and "short row here" (14). The shorter,
    // truncated cell is then padded out to that natural column width like
    // any other cell.
    let expected = "\
a very lo…     x
short row here y";
    assert_eq!(table.render(), expected);
}

#[test]
fn table_wide_ellipsis_applies_to_cell_truncate_too() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.ellipsis("...");
        t.body(|b| {
            b.row(|r| {
                r.cell("a very long piece of text").truncate(10);
            });
        });
    });

    assert_eq!(table.render(), "a very ...");
}

#[test]
fn table_wide_ellipsis_applies_to_column_width_truncation_too() {
    let table = Table::build(|t| {
        t.table_borders(Borders::NONE);
        t.ellipsis("~");
        t.body(|b| {
            b.column(0).max_width(5);
            b.row_cells(["a very long value"]);
        });
    });

    assert_eq!(table.render(), "a ve~");
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
