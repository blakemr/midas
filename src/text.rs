//! Struct to manage the document text
use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Result};
use std::path::Path;

/// Text struct
///
/// parameters:
///     * pos: usize - cursor position in text
///     * text: Rope - current document
///     * path: String - document filepath
pub struct Text {
    pos: usize,
    path: String,

    pub text: Rope,
}

impl Text {
    /// Creates a new, empty, unnamed Text.
    pub fn new() -> Text {
        Text {
            pos: 0,
            text: Rope::new(),
            path: String::new(),
        }
    }

    /// clears the text, resets the cursor position.
    fn clear(&mut self) {
        self.text = Rope::new();
        self.pos = 0;
    }

    /// Reads text into the current Text given a filepath
    ///
    /// args:
    ///     * f: String - Filepath to read from. If the file does not exist,
    ///         a new Text is made with this path as its name.
    fn open_file(&mut self, f: String) -> Result<()> {
        if Path::new(&f).exists() {
            self.text = Rope::from_reader(BufReader::new(File::open(&f)?))?;
            self.path = f;
            self.pos = 0;
        } else {
            self.path = f;
            self.clear();
        }
        Ok()
    }

    /// Saves the current document into a file.
    fn save(&mut self) -> Result<()> {
        // Throw error if the current document is unnamed
        // TODO: Automatically promopt the user to name the file.
        if self.path.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Cannot write a file without a name",
            ));
        }

        // Write the document into the file.
        // TODO: It might be better to only partially rewrite the
        //       file if it's been saved before
        let mut file = File::create(&self.name)?;
        for chunk in self.text.chunks() {
            write!(file, "{}", chunk)?;
        }

        file.sync_all()?;
        Ok(())
    }

    /// Renames the current file
    fn rename(&mut self, path: String) {
        self.path = path;
    }
}
