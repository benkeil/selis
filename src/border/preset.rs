//! Named border presets combining a [`BorderChars`] set (and, optionally, a
//! distinct set used for the line separating header/body/footer sections).

use crate::border::chars::BorderChars;

/// A complete border style: the glyphs used for the regular grid, plus an
/// optional distinct glyph set for the separator line between sections
/// (header/body/footer), matching Mordant's `SQUARE_DOUBLE_SECTION_SEPARATOR`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderPreset {
    pub chars: BorderChars,
    pub section_separator_chars: Option<BorderChars>,
}

impl BorderPreset {
    /// Builds a fully custom preset, e.g. mixing your own [`BorderChars`]
    /// with (optionally) a distinct section-separator line.
    pub const fn custom(chars: BorderChars, section_separator_chars: Option<BorderChars>) -> Self {
        BorderPreset {
            chars,
            section_separator_chars,
        }
    }

    /// No visible border glyphs at all (see [`crate::theme::github`] for the
    /// borderless GitHub-style preset, which uses this).
    pub const fn none() -> Self {
        BorderPreset::custom(BorderChars::NONE, None)
    }

    /// Plain ASCII borders (`-`, `|`, `+`).
    pub const fn ascii() -> Self {
        BorderPreset::custom(BorderChars::ASCII, None)
    }

    /// Light single-line Unicode box-drawing borders.
    pub const fn utf8_square() -> Self {
        BorderPreset::custom(BorderChars::UTF8_SQUARE, None)
    }

    /// Rounded-corner Unicode box-drawing borders.
    pub const fn utf8_rounded() -> Self {
        BorderPreset::custom(BorderChars::UTF8_ROUNDED, None)
    }

    /// Light single-line borders everywhere, except the line(s) separating
    /// header/body/footer sections, which use double-line characters.
    /// Mirrors Mordant's `SQUARE_DOUBLE_SECTION_SEPARATOR`.
    pub const fn square_double_section_separator() -> Self {
        BorderPreset::custom(BorderChars::UTF8_SQUARE, Some(BorderChars::UTF8_DOUBLE))
    }

    /// The glyph set to use for a section-separator line, falling back to
    /// the regular glyph set if no distinct one was configured.
    pub fn section_separator_chars(&self) -> BorderChars {
        self.section_separator_chars.unwrap_or(self.chars)
    }
}

impl Default for BorderPreset {
    fn default() -> Self {
        BorderPreset::utf8_square()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_separator_chars_falls_back_to_regular_chars() {
        let preset = BorderPreset::utf8_square();
        assert_eq!(preset.section_separator_chars(), BorderChars::UTF8_SQUARE);
    }

    #[test]
    fn square_double_section_separator_uses_double_lines_for_separator() {
        let preset = BorderPreset::square_double_section_separator();
        assert_eq!(preset.chars, BorderChars::UTF8_SQUARE);
        assert_eq!(preset.section_separator_chars(), BorderChars::UTF8_DOUBLE);
    }

    #[test]
    fn default_is_utf8_square() {
        assert_eq!(BorderPreset::default().chars, BorderChars::UTF8_SQUARE);
    }
}
