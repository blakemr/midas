//! # Handles Inputs related to the various input modes

/// Sets up the modes of operation. These are named after the standards
/// set by vim, but may be changed slightly since the modal commands are intended
/// to work with a gui from the ground up.
pub enum Mode {
    Normal,
    Insert,
    Command,
    Exit,
}
