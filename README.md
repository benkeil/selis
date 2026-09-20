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
  builders for header/body/footer sections, rows, columns, and cells.
- **Cascading style system** — foreground/background color, bold, italic,
  underline, dim, alignment, and uppercase/lowercase transforms resolve from
  table → section → column → row → cell, each level only overriding what it
  explicitly sets. `.exact()` resets the cascade at that level, discarding
  everything inherited so far.
- **True colors** — 16 ANSI colors, 256-color palette indices, full 24-bit
  RGB, and `#rrggbb` hex parsing.
- **Colspan & rowspan** — merge cells across columns and rows; column widths
  are computed to fit spanning content without needlessly widening every
  column.
- **Per-column overrides** — scoped to one section, or table-wide (an
  "Excel-style" whole-column selection) for style, borders, and width.
- **Configurable borders** — per-cell/row/column/section/table border-side
  flags (`Borders::TOP`, `LEFT_BOTTOM`, …), built-in glyph presets (`ascii`,
  `utf8_square`, `utf8_rounded`, `square_double_section_separator`, `none`),
  or fully custom `BorderChars`.
- **Themes** — bundle a border preset, table style, and header style in one
  call (`plain`, `github`, `github_zebra`, `classic`, `rounded`,
  `minimal_zebra`), still overridable at any more specific level afterwards.
- **Zebra striping** — alternating row styles by position within a section,
  settable directly or pre-populated by a theme.
- **Column width control** — `min_width`/`width`/`max_width` per column,
  plus `cap_to_width` to shrink the whole table's columns down (widest
  first) so it never exceeds a target width — opt-in, off by default. A
  single cell can also be truncated immediately via `.truncate(n)`.
- **Optional header/footer rendering** — `show_header`/`show_footer` toggle
  whether those sections are emitted, without touching their configuration.
- **Captions** — a plain string rendered above/below the table.

## Installation

```toml
[dependencies]
selis = "0.1"
```

## Usage

### The style cascade

Every builder level (`table`/`header`/`body`/`footer`/`column`/`row`/`cell`)
exposes the same `fg`/`bg`/`bold`/`italic`/`underline`/`align` shortcuts, so a
setting can be pinned at whichever level it should apply from. A more
specific level always wins over a less specific one, field by field:

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

`dim`, `no_bold`/`no_italic`/`no_underline`/`no_dim` (to explicitly un-set an
inherited style), `uppercase`/`lowercase` case transforms, and `.exact()`
aren't exposed as builder shortcuts — reach them through `.style(...)`
instead:

```rust
use selis::prelude::*;

let table = Table::build(|t| {
    t.body(|b| {
        b.row_styles([Style::new(), Style::new().dim()]); // zebra striping
        b.row(|r| {
            r.cell("total").style(Style::new().uppercase().exact());
        });
    });
});
```

Colors accept any of the 16 named ANSI colors (`Color::Red`,
`Color::BrightBlue`, …), a 256-color palette index (`Color::ansi256(178)`),
full RGB (`Color::rgb(0x4b, 0x25, 0xb9)`), or a hex string
(`Color::from_hex("#4b25b9")`).

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

### Borders

Border sides are flags (`Borders::TOP`, `Borders::LEFT_BOTTOM`, `Borders::ALL`,
`Borders::NONE`, …), settable at the table/section/column/row/cell level —
the most specific one wins. The glyphs used to draw them are a separate
`BorderPreset`:

```rust
use selis::prelude::*;

let table = Table::build(|t| {
    t.border(BorderPreset::utf8_rounded());
    t.table_borders(Borders::ALL);

    t.body(|b| {
        b.cell_borders(Borders::TOP_BOTTOM);
        b.row_cells(["a", "b"]);
    });
});
```

Built-in presets: `ascii`, `utf8_square`, `utf8_rounded`,
`square_double_section_separator` (single lines, except a double-line
separator between header/body/footer), and `none`. `BorderPreset::custom(...)`
takes a fully custom `BorderChars` set.

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
no header styling, subtle zebra striping — for dense data tables). A theme
just sets table-wide style/border fields, so anything set afterwards
(`t.header(...)`, `t.body(...)`, …) still overrides it, like any other
cascade level.

### Column width: min/max/fixed, and fitting the terminal

Three independent per-column constraints, settable scoped to one section or
table-wide, same as any other column override:

- `max_width(n)` — a ceiling; content that doesn't fit is truncated with a
  trailing `...`.
- `min_width(n)` — a floor; narrower content gets padded out to it.
- `width(n)` — an exact width, ignoring `min_width`/`max_width`: narrower
  content is padded, wider content truncated.

```rust
use selis::prelude::*;

let table = Table::build(|t| {
    t.body(|b| {
        b.column(1).max_width(20);
        b.row_cells(["Description", "A description that runs on for quite a while"]);
    });
});
```

```
┌───────────┬────────────────────┐
│Description│A description tha...│
└───────────┴────────────────────┘
```

A column's width is always shared across header/body/footer (they're laid
out as one grid), so setting a *different* `min_width`/`width`/`max_width`
for the same column from two different sections panics — set it table-wide
via `t.column(n)` instead if it should apply everywhere.

`cap_to_width(n)` goes further: after every column's natural/constrained
width is resolved, if the table as a whole is still wider than `n`, it
shrinks the currently-widest column (down to its `min_width`, or `1` by
default) one character at a time until it fits, or every column has hit its
floor. A table that's already narrower than `n` is left as-is — this only
ever shrinks, never grows. Disabled by default — nothing shrinks unless you
call it:

```rust
use selis::prelude::*;

let table = Table::build(|t| {
    t.cap_to_terminal_width(80); // detect terminal width, fall back to 80
    t.body(|b| {
        b.row_cells(["Description", "A description that runs on for quite a while"]);
    });
});
```

`cap_to_width` itself just takes a plain `usize`, so you can pass any target
(or a fixed constant) instead of `cap_to_terminal_width`'s auto-detection.
Note that the target covers column widths plus the separators between them,
not the table's own outer border characters — leave a couple of characters
of slack if you're rendering with visible left/right borders.

For a one-off, immediate truncation of a single cell's content (rather than
a column-wide constraint), use `.truncate(n)` on the cell instead:

```rust
r.cell("a very long piece of text").truncate(10); // -> "a very ..."
```

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
