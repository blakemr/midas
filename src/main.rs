mod cli;

use cli::Cursor;
use cli::Mode;
use std::io;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

fn main() -> io::Result<()> {
    let mut mode = Mode::Insert;
    let stdin = stdin();
    let mut events = stdin.events();
    let mut termout = stdout().into_raw_mode()?;
    let mut text = String::new();
    let mut command = String::new();

    // Terminal cursor position starts at (1, 1)
    let mut cursor_position: Cursor = Cursor { x: 1, y: 1 };

    // Landing message
    write!(
        termout,
        "{}{}<Esc> to exit. Type stuff, use alt, etc.{}",
        clear::All,
        cursor::Goto(cursor_position.x, cursor_position.y),
        "\r"
    )?;

    termout.flush()?;

    // Main loop.
    loop {
        if let Some(event) = events.next() {
            mode = match mode.handle(event?, &mut command, &mut text, &mut cursor_position) {
                Mode::Exit => break,
                mode => mode,
            }
        }

        write!(
            termout,
            "{}{}{}",
            clear::All,
            cursor::Goto(cursor_position.x, cursor_position.y),
            text
        )?;
        termout.flush()?;
    }

    Ok(())
}
