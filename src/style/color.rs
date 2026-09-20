//! ANSI color representation used by [`crate::style::Style`].
//!
//! Wraps [`anstyle::Color`] with a slightly friendlier surface (named ANSI
//! colors, 256-color palette indices, and full RGB) so users don't need to
//! depend on `anstyle` directly to build a table.

/// A terminal color: one of the 16 classic ANSI colors, a 256-color palette
/// index, or a full 24-bit RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    /// A color from the 256-color ANSI palette (0-255).
    Ansi256(u8),
    /// A full 24-bit RGB color.
    Rgb(u8, u8, u8),
}

impl Color {
    /// Convenience constructor for a full 24-bit RGB color, e.g.
    /// `Color::rgb(0x4b, 0x25, 0xb9)`.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }

    /// Convenience constructor for a 256-color ANSI palette index.
    pub const fn ansi256(index: u8) -> Self {
        Color::Ansi256(index)
    }

    /// Parses a `#rrggbb` (or `rrggbb`) hex string into an RGB color.
    ///
    /// Returns `None` if the string is not a valid 6-digit hex color.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color::Rgb(r, g, b))
    }

    /// Converts this color into the [`anstyle::Color`] used by the renderer.
    pub(crate) fn to_anstyle(self) -> anstyle::Color {
        use anstyle::{Ansi256Color, AnsiColor, Color as A, RgbColor};
        match self {
            Color::Black => A::Ansi(AnsiColor::Black),
            Color::Red => A::Ansi(AnsiColor::Red),
            Color::Green => A::Ansi(AnsiColor::Green),
            Color::Yellow => A::Ansi(AnsiColor::Yellow),
            Color::Blue => A::Ansi(AnsiColor::Blue),
            Color::Magenta => A::Ansi(AnsiColor::Magenta),
            Color::Cyan => A::Ansi(AnsiColor::Cyan),
            Color::White => A::Ansi(AnsiColor::White),
            Color::BrightBlack => A::Ansi(AnsiColor::BrightBlack),
            Color::BrightRed => A::Ansi(AnsiColor::BrightRed),
            Color::BrightGreen => A::Ansi(AnsiColor::BrightGreen),
            Color::BrightYellow => A::Ansi(AnsiColor::BrightYellow),
            Color::BrightBlue => A::Ansi(AnsiColor::BrightBlue),
            Color::BrightMagenta => A::Ansi(AnsiColor::BrightMagenta),
            Color::BrightCyan => A::Ansi(AnsiColor::BrightCyan),
            Color::BrightWhite => A::Ansi(AnsiColor::BrightWhite),
            Color::Ansi256(index) => A::Ansi256(Ansi256Color(index)),
            Color::Rgb(r, g, b) => A::Rgb(RgbColor(r, g, b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_parses_with_and_without_hash() {
        assert_eq!(Color::from_hex("#4b25b9"), Some(Color::Rgb(0x4b, 0x25, 0xb9)));
        assert_eq!(Color::from_hex("4b25b9"), Some(Color::Rgb(0x4b, 0x25, 0xb9)));
    }

    #[test]
    fn from_hex_rejects_invalid_input() {
        assert_eq!(Color::from_hex("#4b25"), None);
        assert_eq!(Color::from_hex("not-a-color"), None);
    }
}
