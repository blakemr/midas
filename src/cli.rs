//! # Manage the CLI version of the text editor
//!
//! This crate is called when midas is run in terminal editor mode.
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

impl Mode {
    /// Handles an input event
    ///
    /// Routes to different handle events based on current mode
    ///
    /// args:
    ///     * event: Event - event to handle
    ///     * command: &mut String - Command buffer. Stored until command is entered
    ///     * text: &mut String - text to be edited
    ///
    /// TODO:
    ///     * Normal Mode
    ///     * Command Mode
    ///     * change text's type?
    pub fn handle(self, event: Event, command: &mut String, text: &mut String) -> Self {
        match self {
            Mode::Normal => Mode::Normal,
            Mode::Insert => Mode::handle_insert(event, text),
            Mode::Command => Mode::Command,
            Mode::Exit => panic!("Unhandled Exit mode."),
        }
    }

    /// Handles events in insert mode
    fn handle_insert(event: Event, text: &mut String) -> Self {
        match event {
            Event::Key(Key::Esc) => {
                return Mode::Exit;
            }
            Event::Key(Key::Char(c)) => {
                text.push(c);
            }
            Event::Key(Key::Backspace) => {
                text.pop();
            }
            // These are placeholder commands
            Event::Key(Key::Alt(c)) => {
                text.push_str("Alt-");
                text.push(c);
            }
            Event::Key(Key::Ctrl(c)) => {
                text.push_str("Ctrl-");
                text.push(c);
            }
            Event::Key(Key::Left) => {
                text.push_str("<Left>");
            }
            Event::Key(Key::Right) => {
                text.push_str("<Right>");
            }
            Event::Key(Key::Up) => {
                text.push_str("<Up>");
            }
            Event::Key(Key::Down) => {
                text.push_str("<Down>");
            }
            _ => {}
        }

        Mode::Insert
    }
}
