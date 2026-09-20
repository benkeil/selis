//! Per-column maximum width, used to truncate overflowing cell content with
//! an ellipsis (`...`) instead of letting the table grow arbitrarily wide.

/// How wide a column is allowed to grow before its content gets truncated
/// (with a trailing `...`) to fit.
///
/// A plain `usize` converts into [`MaxWidth::Fixed`] via [`From`], so
/// `c.max_width(20)` works directly without needing to name the enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaxWidth {
    /// Caps the column at exactly this many terminal columns; content wider
    /// than this is truncated and suffixed with `...`.
    Fixed(usize),
    /// Reserved for a future terminal-width-aware mode that shrinks exactly
    /// this column (and only this column) so the whole table fits the
    /// available terminal width. Not implemented yet — currently has no
    /// effect (behaves the same as not setting a max width at all).
    Auto,
}

impl MaxWidth {
    /// The concrete width limit this resolves to right now, or `None` if it
    /// doesn't (yet) impose one — currently only [`MaxWidth::Fixed`] does;
    /// [`MaxWidth::Auto`] is a no-op placeholder until terminal-width
    /// detection is implemented.
    pub fn resolved_width(self) -> Option<usize> {
        match self {
            MaxWidth::Fixed(width) => Some(width),
            MaxWidth::Auto => None,
        }
    }
}

impl From<usize> for MaxWidth {
    fn from(width: usize) -> Self {
        MaxWidth::Fixed(width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usize_converts_into_fixed() {
        let max_width: MaxWidth = 20.into();
        assert_eq!(max_width, MaxWidth::Fixed(20));
    }

    #[test]
    fn fixed_resolves_to_its_own_width() {
        assert_eq!(MaxWidth::Fixed(20).resolved_width(), Some(20));
    }

    #[test]
    fn auto_does_not_resolve_to_a_width_yet() {
        assert_eq!(MaxWidth::Auto.resolved_width(), None);
    }
}
