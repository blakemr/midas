mod cli;

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

    // Landing message, and hide the cursor
    write!(
        termout,
        "{}{}<Esc> to exit. Type stuff, use alt, etc.{}{}{}",
        clear::All,
        cursor::Goto(1, 1),
        cursor::Down(1),
        "\r",
        cursor::Hide
    )?;

    // Flush to screen
    termout.flush()?;

    loop {
        if let Some(event) = events.next() {
            mode = match mode.handle(event?, &mut command, &mut text) {
                Mode::Exit => break,
                mode => mode,
            }
        }

        write!(termout, "{}{}", text, cursor::Goto(1, 2))?;
        termout.flush()?;
    }

    // Turn the cursor back on
    write!(termout, "{}", cursor::Show)?;

    Ok(())
}
