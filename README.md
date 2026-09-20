# selis

A Kotlin-[Mordant](https://github.com/ajalt/mordant)-inspired, closure-based
DSL for rendering styled terminal tables in Rust.

```rust
use selis::prelude::*;

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

println!("{}", table.render());
```

```
┌───────┬─────┬────────┐
│Name   │Stars│Language│
├───────┼─────┼────────┤
│ratatui│12.3k│Rust    │
├───────┼─────┼────────┤
│mordant│1.2k │Kotlin  │
└───────┴─────┴────────┘
```

## Features

- **Closure-based builder DSL** — `Table::build(|t| { ... })`, with matching
  builders for header/body/footer sections, rows, and cells.
- **Cascading style system** — colors, bold/italic/underline/dim, alignment,
  and case transforms resolve from table → section → column → row → cell,
  each level only overriding what it explicitly sets. An `.exact()` escape
  hatch resets the cascade at any level.
- **Colspan & rowspan** — merge cells across columns and rows; column widths
  are computed to fit spanning content without needlessly widening every
  column.
- **Per-column overrides** — scoped to one section, or table-wide (an
  "Excel-style" whole-column selection) for style, borders, and max width.
- **Configurable borders** — per-cell/row/column/section/table border-side
  flags (`Borders::TOP`, `LEFT_BOTTOM`, …), plus built-in glyph presets
  (`ascii`, `utf8_square`, `utf8_rounded`, `square_double_section_separator`,
  …) or fully custom `BorderChars`.
- **Themes** — bundle border preset + table style + header style in one call
  (`plain`, `github`, `github_zebra`, `classic`, `rounded`, `minimal_zebra`),
  still overridable at any more specific level afterwards.
- **Zebra striping** — alternating row styles by position, settable directly
  or pre-populated by a theme.
- **Max width & truncation** — cap a column's width (ellipsis-truncating
  overflowing content) or truncate an individual cell's content immediately.
- **Optional header/footer rendering** — `show_header`/`show_footer` toggle
  whether those sections are emitted, without touching their configuration.
- **True colors** — 16 ANSI colors, 256-color palette indices, full RGB, and
  `#rrggbb` hex parsing.

## Installation

```toml
[dependencies]
selis = "0.1"
```

## Usage

### Alignment, colors, and styles

Every builder level (`table`/`header`/`body`/`footer`/`column`/`row`/`cell`)
exposes the same shortcuts, so a setting can be pinned at whichever level it
should apply from:

```rust
use selis::prelude::*;

let table = Table::build(|t| {
    t.align(Align::Right);
    t.fg(Color::BrightBlue);

    t.body(|b| {
        b.column(0).align(Align::Left).bold();
        b.row_cells(["Name", "Score"]);
        b.row(|r| {
            r.cell("Alice").fg(Color::Green);
            r.cell("42");
        });
    });
});
```

### Colspan, rowspan, and captions

```rust
use selis::prelude::*;

let table = Table::build(|t| {
    t.body(|b| {
        b.row(|r| {
            r.cell("Category").rowspan(3);
            r.cell("Percent Change").colspan(2).align(Align::Center);
        });
        // ...
    });
    t.caption_bottom("via U.S. Bureau of Labor Statistics");
});
```

### Themes

```rust
use selis::prelude::*;

let table = Table::build(|t| {
    t.theme(github_zebra());
    t.header(|h| {
        h.row_cells(["Status", "Title", "Branch"]);
    });
    t.body(|b| {
        b.row_cells(["✓", "feat: add widget", "main"]);
        b.row_cells(["X", "fix: flaky test", "main"]);
    });
});
```

Built-in themes: `plain` (no borders, no styling), `github`/`github_zebra`
(borderless, uppercase underlined header, optional zebra striping),
`classic` (square borders with a double-line separator between sections,
bold header, matching the original Kotlin/Mordant sample), `rounded`
(rounded-corner borders, bold cyan header), and `minimal_zebra` (borderless,
no header styling, subtle zebra striping — for dense data tables).

### Suppressing the header/footer for CLI-style output

```rust
use selis::prelude::*;

let table = Table::build(|t| {
    t.header(|h| { h.row_cells(["Name", "Stars"]); });
    t.body(|b| { b.row_cells(["ratatui", "12.3k"]); });

    if cli_mode {
        t.show_header(false);
    }
});
```

This only affects rendering — the header's rows/styles/columns stay
configured, so toggling it back to `true` later still renders as before.

### Max width & truncation

```rust
use selis::prelude::*;

let table = Table::build(|t| {
    t.body(|b| {
        b.column(1).max_width(20);
        b.row_cells(["Description", "A description that runs on for quite a while"]);
    });
});
```

## Examples

Run the bundled examples to see the output in your own terminal:

```sh
cargo run --example demo
cargo run --example gh_runs
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
