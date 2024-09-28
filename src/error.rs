use std::fmt::Debug;
use std::path::PathBuf;

use serde_derive::Serialize;

use crate::calender::LineError;

#[derive(Serialize, Debug)]
pub struct Error {
    path: PathBuf,
    errors: Vec<ValueError>,
}

impl Error {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            errors: vec![],
        }
    }

    pub fn push(&mut self, error: ValueError) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn format(self) -> String {
        let cap = self.errors.len();
        self.errors
            .into_iter()
            .fold(String::with_capacity(cap * 100), |acc, err| {
                format!(
                    "{acc}error: -> {}{}",
                    self.path.to_string_lossy(),
                    err.format()
                )
            })
    }
}

#[derive(Serialize, Debug)]
pub struct ValueError {
    line_error: LineError,
    line_number: usize,
    line: String,
}

impl ValueError {
    pub fn new(line_error: LineError, line_number: usize, line: String) -> Self {
        Self {
            line_error,
            line_number,
            line,
        }
    }

    pub fn format(self) -> String {
        match self.line_error {
            LineError::CantParseInput => cant_parse_input(self.line_number, self.line),
            LineError::InvalidValue(m) => invalid_value(m, self.line_number, self.line),
            LineError::ParsingError { error, .. } => format!("{error}\n\n"),
        }
    }
}

fn invalid_value(message: String, line_number: usize, line: String) -> String {
    let line_number = line_number.to_string();
    let spacing = " ".repeat(line_number.len());

    format!(
        ":{ls:w$}\n{s} |\n\
         {ls:w$} | {line}\n\
         {s} |\n\
         {s} = note: {message}
\n\n",
        s = spacing,
        ls = line_number,
        w = spacing.len()
    )
}

fn cant_parse_input(line_number: usize, line: String) -> String {
    let line_number = line_number.to_string();
    let spacing = " ".repeat(line_number.len());

    format!(
        ":{ls:w$}\n{s} |\n\
         {ls:w$} | {line}\n\
         {s} |\n\
         {s} = note: can't parse input.
\n\n",
        s = spacing,
        ls = line_number,
        w = spacing.len()
    )
}

// The name `io` is not a good idea, it being used in the standard library.
// So instead of judging please suggest a better name.
// For now I'm renaming the `Error` struct to `FileError`.
pub mod io {
    use std::{error::Error as StdError, fmt, path::PathBuf};

    #[derive(thiserror::Error)]
    #[error("I/O error while processing file: {file}")]
    pub struct FileError {
        #[source]
        source: std::io::Error,
        file: PathBuf,
    }

    impl FileError {
        pub fn new(file: PathBuf, source: std::io::Error) -> Self {
            FileError { source, file }
        }
    }

    impl fmt::Debug for FileError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} ", self)?;

            if let Some(source) = self.source() {
                write!(f, "Caused by:\n\t{}", source)?;
            }

            Ok(())
        }
    }
}
