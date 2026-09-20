//! The cascading [`Style`] type.
//!
//! A [`Style`] only ever describes *overrides*: every field is an
//! [`Option`], so "not set" and "set" are distinguishable. This is what makes
//! the style cascade possible — a `Style` defined on the [`crate::model::table::Table`]
//! (least specific) is merged with increasingly specific styles from
//! section, column, row, and finally cell (most specific), with the more
//! specific `Some(..)` value always winning.
//!
//! If a user wants a style to ignore everything inherited from above and be
//! applied as-is, they can mark it with [`Style::exact`]. When
//! [`Style::cascade`] encounters an exact style it resets the accumulated
//! result to that style before continuing to merge any more specific levels
//! on top of it.

use crate::style::align::Align;
use crate::style::case::Case;
use crate::style::color::Color;

/// A set of style overrides (colors, text effects, alignment) that can be
/// merged into a cascade of increasingly specific styles.
///
/// All fields are optional: `None` means "inherit from a less specific
/// level", `Some(..)` means "set (or unset) this at this level".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
    pub(crate) bold: Option<bool>,
    pub(crate) italic: Option<bool>,
    pub(crate) underline: Option<bool>,
    pub(crate) dim: Option<bool>,
    pub(crate) align: Option<Align>,
    pub(crate) case: Option<Case>,
    pub(crate) exact: bool,
}

impl Style {
    /// Creates an empty style (no overrides set).
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the foreground (text) color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets the background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Enables bold text.
    pub fn bold(mut self) -> Self {
        self.bold = Some(true);
        self
    }

    /// Explicitly disables bold text, overriding any inherited bold.
    pub fn no_bold(mut self) -> Self {
        self.bold = Some(false);
        self
    }

    /// Enables italic text.
    pub fn italic(mut self) -> Self {
        self.italic = Some(true);
        self
    }

    /// Explicitly disables italic text, overriding any inherited italic.
    pub fn no_italic(mut self) -> Self {
        self.italic = Some(false);
        self
    }

    /// Enables underlined text.
    pub fn underline(mut self) -> Self {
        self.underline = Some(true);
        self
    }

    /// Explicitly disables underlined text, overriding any inherited underline.
    pub fn no_underline(mut self) -> Self {
        self.underline = Some(false);
        self
    }

    /// Enables dimmed ("faint") text.
    pub fn dim(mut self) -> Self {
        self.dim = Some(true);
        self
    }

    /// Explicitly disables dimmed text, overriding any inherited dim.
    pub fn no_dim(mut self) -> Self {
        self.dim = Some(false);
        self
    }

    /// Sets the horizontal alignment.
    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    /// Transforms this cell's content to uppercase before rendering.
    pub fn uppercase(mut self) -> Self {
        self.case = Some(Case::Uppercase);
        self
    }

    /// Transforms this cell's content to lowercase before rendering.
    pub fn lowercase(mut self) -> Self {
        self.case = Some(Case::Lowercase);
        self
    }

    /// Marks this style as "exact": when applied in a cascade, it discards
    /// everything inherited so far instead of merging with it. Levels more
    /// specific than this one can still override individual fields on top.
    pub fn exact(mut self) -> Self {
        self.exact = true;
        self
    }

    /// Merges `other` on top of `self`: every field that is `Some(..)` in
    /// `other` wins; every field that is `None` in `other` falls back to
    /// `self`. `other`'s [`exact`](Style::exact) flag is not consulted here —
    /// use [`Style::cascade`] to resolve a full cascade including exact
    /// resets.
    pub fn merge(&self, other: &Style) -> Style {
        Style {
            fg: other.fg.or(self.fg),
            bg: other.bg.or(self.bg),
            bold: other.bold.or(self.bold),
            italic: other.italic.or(self.italic),
            underline: other.underline.or(self.underline),
            dim: other.dim.or(self.dim),
            align: other.align.or(self.align),
            case: other.case.or(self.case),
            exact: false,
        }
    }

    /// Resolves a full cascade of styles, from least specific (e.g. table)
    /// to most specific (e.g. cell). `None` entries (an unset level) are
    /// skipped. A level marked [`exact`](Style::exact) resets the result to
    /// that style before continuing, so it fully overrides everything above
    /// it while still allowing more specific levels to override it further.
    pub fn cascade<'a>(levels: impl IntoIterator<Item = Option<&'a Style>>) -> Style {
        let mut resolved = Style::default();
        for level in levels.into_iter().flatten() {
            resolved = if level.exact {
                let mut reset = *level;
                reset.exact = false;
                reset
            } else {
                resolved.merge(level)
            };
        }
        resolved
    }

    /// Converts the resolved style into an [`anstyle::Style`] usable for
    /// rendering (colors + bold/italic/underline/dim effects). `None`/`Some(false)`
    /// fields are treated as "no effect".
    pub fn to_anstyle(self) -> anstyle::Style {
        let mut style = anstyle::Style::new();
        if let Some(fg) = self.fg {
            style = style.fg_color(Some(fg.to_anstyle()));
        }
        if let Some(bg) = self.bg {
            style = style.bg_color(Some(bg.to_anstyle()));
        }
        let mut effects = anstyle::Effects::new();
        if self.bold == Some(true) {
            effects |= anstyle::Effects::BOLD;
        }
        if self.italic == Some(true) {
            effects |= anstyle::Effects::ITALIC;
        }
        if self.underline == Some(true) {
            effects |= anstyle::Effects::UNDERLINE;
        }
        if self.dim == Some(true) {
            effects |= anstyle::Effects::DIMMED;
        }
        style.effects(effects)
    }

    /// Renders `text` wrapped in the ANSI escape codes for this style (and
    /// the reset sequence afterwards). Useful for one-off styled strings
    /// (e.g. table captions) outside of the table model itself.
    pub fn render(self, text: &str) -> String {
        let ansi = self.to_anstyle();
        format!("{ansi}{text}{ansi:#}")
    }

    /// The resolved horizontal alignment, defaulting to [`Align::Left`] if unset.
    pub fn resolved_align(self) -> Align {
        self.align.unwrap_or_default()
    }

    /// The resolved foreground color, if any.
    pub fn resolved_fg(self) -> Option<Color> {
        self.fg
    }

    /// The resolved background color, if any.
    pub fn resolved_bg(self) -> Option<Color> {
        self.bg
    }

    /// The resolved bold flag (`None` means "unset").
    pub fn resolved_bold(self) -> Option<bool> {
        self.bold
    }

    /// The resolved italic flag (`None` means "unset").
    pub fn resolved_italic(self) -> Option<bool> {
        self.italic
    }

    /// The resolved underline flag (`None` means "unset").
    pub fn resolved_underline(self) -> Option<bool> {
        self.underline
    }

    /// The resolved dim flag (`None` means "unset").
    pub fn resolved_dim(self) -> Option<bool> {
        self.dim
    }

    /// The resolved text-case transform, if any.
    pub fn resolved_case(self) -> Option<Case> {
        self.case
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_lets_more_specific_fields_win() {
        let table = Style::new().fg(Color::Green);
        let row = Style::new().fg(Color::Red);
        let merged = table.merge(&row);
        assert_eq!(merged.fg, Some(Color::Red));
    }

    #[test]
    fn merge_falls_back_to_less_specific_when_unset() {
        let table = Style::new().fg(Color::Green).bold();
        let row = Style::new().fg(Color::Red);
        let merged = table.merge(&row);
        assert_eq!(merged.fg, Some(Color::Red));
        assert_eq!(merged.bold, Some(true));
    }

    #[test]
    fn cascade_resolves_table_section_column_row_cell_in_order() {
        let table = Style::new().fg(Color::Green);
        let body = Style::new().bold();
        let column = Style::new().fg(Color::BrightBlue);
        let row = Style::new().fg(Color::Red);
        let cell = Style::new().italic();

        let resolved =
            Style::cascade([Some(&table), Some(&body), Some(&column), Some(&row), Some(&cell)]);

        // row's fg wins over column's and table's.
        assert_eq!(resolved.fg, Some(Color::Red));
        // bold from body is inherited since nothing below overrides it.
        assert_eq!(resolved.bold, Some(true));
        // italic set at cell level.
        assert_eq!(resolved.italic, Some(true));
    }

    #[test]
    fn none_levels_are_skipped() {
        let table = Style::new().fg(Color::Green);
        let resolved = Style::cascade([Some(&table), None, None, None, None]);
        assert_eq!(resolved.fg, Some(Color::Green));
    }

    #[test]
    fn exact_style_discards_everything_inherited_so_far() {
        let table = Style::new().fg(Color::Green).bold().italic();
        let cell = Style::new().fg(Color::White).exact();

        let resolved = Style::cascade([Some(&table), Some(&cell)]);

        assert_eq!(resolved.fg, Some(Color::White));
        assert_eq!(resolved.bold, None);
        assert_eq!(resolved.italic, None);
    }

    #[test]
    fn levels_more_specific_than_exact_can_still_override_on_top() {
        let table = Style::new().fg(Color::Green).bold();
        let column = Style::new().fg(Color::White).exact();
        let row = Style::new().fg(Color::Red);

        let resolved = Style::cascade([Some(&table), Some(&column), Some(&row)]);

        // row overrides the exact column's fg...
        assert_eq!(resolved.fg, Some(Color::Red));
        // ...but bold from table does not leak through the exact reset.
        assert_eq!(resolved.bold, None);
    }

    #[test]
    fn no_bold_explicitly_overrides_an_inherited_bold() {
        let table = Style::new().bold();
        let cell = Style::new().no_bold();
        let resolved = Style::cascade([Some(&table), Some(&cell)]);
        assert_eq!(resolved.bold, Some(false));
    }

    #[test]
    fn uppercase_and_lowercase_case_cascade_like_other_fields() {
        let section = Style::new().uppercase();
        let cell = Style::new().lowercase();

        assert_eq!(Style::cascade([Some(&section)]).resolved_case(), Some(Case::Uppercase));
        assert_eq!(
            Style::cascade([Some(&section), Some(&cell)]).resolved_case(),
            Some(Case::Lowercase)
        );
    }
}
