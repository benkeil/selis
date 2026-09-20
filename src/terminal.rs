//! Terminal-width detection, for use with
//! [`crate::builder::table_builder::TableBuilder::cap_to_width`].

/// The current terminal's width in columns, or `None` if it can't be
/// determined (e.g. stdout isn't a terminal). A thin wrapper around the
/// `terminal_size` crate.
///
/// Not called automatically by [`crate::model::table::Table::render`] —
/// capping a table to the terminal's width is always an explicit opt-in via
/// [`crate::builder::table_builder::TableBuilder::cap_to_width`], so
/// rendering stays deterministic by default.
pub fn terminal_width() -> Option<usize> {
    terminal_size::terminal_size().map(|(width, _)| width.0 as usize)
}
