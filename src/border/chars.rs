//! The set of box-drawing characters used to render a table's borders.
//!
//! A [`BorderChars`] describes the glyphs for straight segments, corners,
//! T-junctions, and the full cross — everything needed to join borders of
//! neighboring cells into a single, correctly-connected grid (similar to
//! CSS's `border-collapse`). See [`crate::layout::border_grid`] (implemented
//! in a later phase) for how neighboring cells' [`crate::border::Borders`]
//! flags are combined to decide, at each grid vertex, which of the four
//! directions (up/right/down/left) are present — [`BorderChars::junction`]
//! then picks the matching glyph.

/// A named set of box-drawing characters for one border style.
///
/// Every field can be set independently, so fully custom border styles are
/// possible (e.g. mixing heavy verticals with light horizontals).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderChars {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    /// `┬` — horizontal line with a branch going down (top edge of an inner column divider).
    pub t_down: char,
    /// `┴` — horizontal line with a branch going up (bottom edge of an inner column divider).
    pub t_up: char,
    /// `├` — vertical line with a branch going right (left edge of an inner row divider).
    pub t_right: char,
    /// `┤` — vertical line with a branch going left (right edge of an inner row divider).
    pub t_left: char,
    /// `┼` — full cross, all four directions present.
    pub cross: char,
}

impl BorderChars {
    /// All-space border characters — used by [`crate::border::BorderPreset::none`]
    /// so a "border grid" can still be computed uniformly, but nothing
    /// visible is drawn.
    pub const NONE: BorderChars = BorderChars {
        horizontal: ' ',
        vertical: ' ',
        top_left: ' ',
        top_right: ' ',
        bottom_left: ' ',
        bottom_right: ' ',
        t_down: ' ',
        t_up: ' ',
        t_right: ' ',
        t_left: ' ',
        cross: ' ',
    };

    /// Plain ASCII border characters (`-`, `|`, `+`), for terminals without
    /// Unicode box-drawing support.
    pub const ASCII: BorderChars = BorderChars {
        horizontal: '-',
        vertical: '|',
        top_left: '+',
        top_right: '+',
        bottom_left: '+',
        bottom_right: '+',
        t_down: '+',
        t_up: '+',
        t_right: '+',
        t_left: '+',
        cross: '+',
    };

    /// Light single-line Unicode box-drawing characters (`─`, `│`, `┌`, …).
    pub const UTF8_SQUARE: BorderChars = BorderChars {
        horizontal: '─',
        vertical: '│',
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
        t_down: '┬',
        t_up: '┴',
        t_right: '├',
        t_left: '┤',
        cross: '┼',
    };

    /// Rounded-corner Unicode box-drawing characters (`╭`, `╮`, `╰`, `╯`, …).
    pub const UTF8_ROUNDED: BorderChars = BorderChars {
        horizontal: '─',
        vertical: '│',
        top_left: '╭',
        top_right: '╮',
        bottom_left: '╰',
        bottom_right: '╯',
        t_down: '┬',
        t_up: '┴',
        t_right: '├',
        t_left: '┤',
        cross: '┼',
    };

    /// Double-line Unicode box-drawing characters (`═`, `║`, `╔`, …), used
    /// e.g. as the section-separator line between header/body/footer.
    pub const UTF8_DOUBLE: BorderChars = BorderChars {
        horizontal: '═',
        vertical: '║',
        top_left: '╔',
        top_right: '╗',
        bottom_left: '╚',
        bottom_right: '╝',
        t_down: '╦',
        t_up: '╩',
        t_right: '╠',
        t_left: '╣',
        cross: '╬',
    };

    /// Mixed glyphs for a *double horizontal* line crossing *single*
    /// vertical lines — used e.g. as the section-separator line between
    /// header/body/footer, whose verticals stay single (`│`) while only the
    /// separator itself is doubled (`═`). Unlike [`BorderChars::UTF8_DOUBLE`]
    /// (which is fully double, including the verticals), this uses the
    /// dedicated "single vertical / double horizontal" Unicode glyphs
    /// (`╪ ╤ ╧ ╞ ╡` and corners `╒ ╕ ╘ ╛`).
    pub const UTF8_DOUBLE_HORIZONTAL: BorderChars = BorderChars {
        horizontal: '═',
        vertical: '│',
        top_left: '╒',
        top_right: '╕',
        bottom_left: '╘',
        bottom_right: '╛',
        t_down: '╤',
        t_up: '╧',
        t_right: '╞',
        t_left: '╡',
        cross: '╪',
    };

    /// Picks the correct glyph for a grid vertex given which of the four
    /// directions (up/right/down/left) have a border line entering it.
    ///
    /// A vertex with only one direction present (a "stub", e.g. a border
    /// segment ending mid-grid) degrades to the corresponding straight
    /// segment (`vertical` for up/down stubs, `horizontal` for left/right
    /// stubs), since most presets don't define distinct half-line glyphs.
    pub fn junction(&self, up: bool, right: bool, down: bool, left: bool) -> char {
        match (up, right, down, left) {
            (false, false, false, false) => ' ',
            (true, false, false, false) | (false, false, true, false) | (true, false, true, false) => {
                self.vertical
            }
            (false, true, false, false) | (false, false, false, true) | (false, true, false, true) => {
                self.horizontal
            }
            (false, true, true, false) => self.top_left,
            (false, false, true, true) => self.top_right,
            (true, true, false, false) => self.bottom_left,
            (true, false, false, true) => self.bottom_right,
            (false, true, true, true) => self.t_down,
            (true, true, false, true) => self.t_up,
            (true, true, true, false) => self.t_right,
            (true, false, true, true) => self.t_left,
            (true, true, true, true) => self.cross,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn junction_picks_straight_segments() {
        let c = BorderChars::UTF8_SQUARE;
        assert_eq!(c.junction(true, false, true, false), '│');
        assert_eq!(c.junction(false, true, false, true), '─');
    }

    #[test]
    fn junction_picks_corners() {
        let c = BorderChars::UTF8_SQUARE;
        assert_eq!(c.junction(false, true, true, false), '┌');
        assert_eq!(c.junction(false, false, true, true), '┐');
        assert_eq!(c.junction(true, true, false, false), '└');
        assert_eq!(c.junction(true, false, false, true), '┘');
    }

    #[test]
    fn junction_picks_t_junctions_and_cross() {
        let c = BorderChars::UTF8_SQUARE;
        assert_eq!(c.junction(false, true, true, true), '┬');
        assert_eq!(c.junction(true, true, false, true), '┴');
        assert_eq!(c.junction(true, true, true, false), '├');
        assert_eq!(c.junction(true, false, true, true), '┤');
        assert_eq!(c.junction(true, true, true, true), '┼');
    }

    #[test]
    fn junction_degrades_stubs_to_axis_line() {
        let c = BorderChars::UTF8_SQUARE;
        assert_eq!(c.junction(true, false, false, false), '│');
        assert_eq!(c.junction(false, true, false, false), '─');
    }

    #[test]
    fn junction_of_no_directions_is_space() {
        assert_eq!(BorderChars::UTF8_SQUARE.junction(false, false, false, false), ' ');
    }

    #[test]
    fn double_horizontal_junction_keeps_single_verticals() {
        let c = BorderChars::UTF8_DOUBLE_HORIZONTAL;
        assert_eq!(c.junction(true, true, true, true), '╪');
        assert_eq!(c.junction(false, true, true, true), '╤');
        assert_eq!(c.junction(true, true, false, true), '╧');
        assert_eq!(c.junction(true, true, true, false), '╞');
        assert_eq!(c.junction(true, false, true, true), '╡');
        assert_eq!(c.junction(false, true, true, false), '╒');
        assert_eq!(c.junction(false, false, true, true), '╕');
        assert_eq!(c.junction(true, true, false, false), '╘');
        assert_eq!(c.junction(true, false, false, true), '╛');
    }
}
