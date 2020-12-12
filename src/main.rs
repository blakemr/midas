mod cli;

use cli::Mode;
use std::io;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

fn main() -> io::Result<()> {
    let mut mode = Mode::Normal;
    let stdin = stdin();
    let mut events = stdin.events();
    let mut termout = stdout().into_raw_mode()?;
    let mut text = String::new();
    let mut command = String::new();

    // Landing message
    text = "<Esc> to exit. Type stuff.".to_string();
    write!(termout, "{}{}{}", clear::All, cursor::Goto(1, 1), text)?;
    termout.flush()?;

    // Main loop.
    loop {
        if let Some(event) = events.next() {
            mode = match mode.handle(event?, &mut command, &mut text) {
                Mode::Exit => break,
                mode => mode,
            }
        }

        write!(termout, "{}{}{}", clear::All, cursor::Goto(1, 1), text)?;
        termout.flush()?;
    }

    Ok(())
}
