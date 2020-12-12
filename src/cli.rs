//! # Manage the CLI version of the text editor
//!
//! This crate is called when midas is run in terminal editor mode.

use crate::modes::Mode;
use termion::event::{Event, Key};

use std::fs::File;
use std::io::Write;

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
            Mode::Normal => Mode::handle_normal(event, text),
            Mode::Insert => Mode::handle_insert(event, text),
            Mode::Command => Mode::Command,
            Mode::Exit => panic!("Unhandled Exit mode."),
        }
    }

    /// Handles events in insert mode
    fn handle_insert(event: Event, text: &mut String) -> Self {
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
    fn handle_normal(event: Event, text: &mut String) -> Self {
        match event {
            Event::Key(Key::Char('i')) => {
                return Mode::Insert;
            }
            Event::Key(Key::Esc) => {
                return Mode::Exit;
            }
            Event::Key(Key::Char('w')) => {
                let mut file = File::create("foo.txt").unwrap();
                write!(file, "{}", text);
            }
            _ => {}
        }

        Mode::Normal
    }
}
