//! `selis` — a Kotlin-Mordant-inspired, closure-based DSL for rendering
//! styled terminal tables.
//!
//! This crate is under active construction; the module layout below reflects
//! the intended architecture and will be filled in incrementally:
//!
//! - [`model`]: pure data structures (`Table`, `Section`, `Row`, `Cell`, `Column`).
//! - [`style`]: `Color`, `Align`, and the cascading `Style` type.
//! - [`border`]: border character sets, presets, and per-cell border flags.
//! - `theme`: built-in presets (e.g. GitHub style) built on top of `style`/`border`.
//! - `layout`: colspan/rowspan grid resolution and column width computation.
//! - `render`: turns a resolved model + layout into a final ANSI string.
//! - `builder`: the public closure-based DSL used to construct a [`model::table::Table`].

pub mod border;
pub mod builder;
pub mod layout;
pub mod model;
pub mod render;
pub mod style;
mod text;
pub mod theme;

pub use border::{BorderChars, BorderPreset, Borders};
pub use model::{Cell, Column, MaxWidth, Row, SectionKind, Table};
pub use style::{Align, Case, Color, Style};
pub use theme::Theme;

/// Convenient glob-import of the most commonly used types.
pub mod prelude {
    pub use crate::border::{BorderChars, BorderPreset, Borders};
    pub use crate::model::{Cell, Column, MaxWidth, Row, SectionKind, Table};
    pub use crate::style::{Align, Case, Color, Style};
    pub use crate::theme::{
        Theme, classic::classic, github::github, github::github_zebra,
        minimal_zebra::minimal_zebra, plain::plain, rounded::rounded,
    };
}
