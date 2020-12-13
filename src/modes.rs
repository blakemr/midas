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
    pub fn handle(self, event: Event, text: &mut Text) -> Self {
        match self {
            Mode::Normal => Mode::handle_normal(event, text),
            Mode::Insert => Mode::handle_insert(event, text),
            Mode::Exit => panic!("Unhandled Exit mode."),
        }
    }

    /// Handles events in insert mode
    fn handle_insert(event: Event, text: &mut Text) -> Self {
        match event {
            Event::Key(Key::Esc) => {
                return Mode::Normal;
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

    // Handles events in normal modecursor: &mut Cursor
    fn handle_normal(event: Event, text: &mut Text) -> Self {
        match event {
            Event::Key(Key::Char('i')) => {
                return Mode::Insert;
            }
            Event::Key(Key::Esc) => {
                return Mode::Exit;
            }
            Event::Key(Key::Char('w')) => {
                text.save();
            }
            _ => {}
        }

        Mode::Normal
    }
}
