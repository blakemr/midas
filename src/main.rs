mod cli;
mod modes;
mod text;

use modes::Mode;
use std::io;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};
use text::Text;

fn main() -> io::Result<()> {
    let mut mode = Mode::Normal;

    let stdin = stdin();
    let mut events = stdin.events();
    let mut termout = stdout().into_raw_mode()?;

    let mut text = Text::new();
    let mut command = String::new();

    // Landing message
    write!(
        termout,
        "{}{}<Esc> to exit. Type stuff.",
        clear::All,
        cursor::Goto(1, 1),
    )?;
    termout.flush()?;

    // Main loop.
    loop {
        if let Some(event) = events.next() {
            mode = match mode.handle(event?, &mut command, &mut text) {
                Mode::Exit => break,
                mode => mode,
            }
        }

        write!(termout, "{}{}", clear::All, cursor::Goto(1, 1));
        for line in text.text.lines() {
            write!(termout, "{}", line);
        }
        termout.flush()?;
    }

    Ok(())
}
