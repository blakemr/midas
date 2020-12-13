//! # Handles Inputs related to the various input modes
use crate::text::Text;

// Look, I don't want Mode tied to termion, but I don't want to spend the time to figure out the generic
// version when I'm not even going to use termion in the final version anyway.
use termion::event::{Event, Key};

/// Sets up the modes of operation. These are named after the standards
/// set by vim, but may be changed slightly since the modal commands are intended
/// to work with a gui from the ground up.
pub enum Mode {
    Normal,
    Insert,
    Command,
    Exit,
}
