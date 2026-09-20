//! Which sides of a cell/row/column should draw a border, as combinable flags.

use bitflags::bitflags;

bitflags! {
    /// Which side(s) of a cell a border should be drawn on. Combine with `|`,
    /// e.g. `Borders::TOP | Borders::BOTTOM` (equivalent to the provided
    /// [`Borders::TOP_BOTTOM`] shorthand).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Borders: u8 {
        const TOP    = 0b0001;
        const RIGHT  = 0b0010;
        const BOTTOM = 0b0100;
        const LEFT   = 0b1000;

        const NONE = 0;
        const ALL = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
        const TOP_BOTTOM = Self::TOP.bits() | Self::BOTTOM.bits();
        const LEFT_RIGHT = Self::LEFT.bits() | Self::RIGHT.bits();
        const LEFT_BOTTOM = Self::LEFT.bits() | Self::BOTTOM.bits();
        const RIGHT_BOTTOM = Self::RIGHT.bits() | Self::BOTTOM.bits();
        const LEFT_TOP = Self::LEFT.bits() | Self::TOP.bits();
        const RIGHT_TOP = Self::RIGHT.bits() | Self::TOP.bits();
    }
}

impl Default for Borders {
    fn default() -> Self {
        Borders::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shorthands_match_manual_combinations() {
        assert_eq!(Borders::TOP_BOTTOM, Borders::TOP | Borders::BOTTOM);
        assert_eq!(Borders::ALL, Borders::TOP | Borders::RIGHT | Borders::BOTTOM | Borders::LEFT);
    }

    #[test]
    fn contains_checks_individual_sides() {
        let b = Borders::LEFT_BOTTOM;
        assert!(b.contains(Borders::LEFT));
        assert!(b.contains(Borders::BOTTOM));
        assert!(!b.contains(Borders::TOP));
        assert!(!b.contains(Borders::RIGHT));
    }

    #[test]
    fn default_is_none() {
        assert_eq!(Borders::default(), Borders::NONE);
    }
}
