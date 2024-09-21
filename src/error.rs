use std::fmt;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

use serde_derive::Serialize;

use crate::parser::Rule;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Serialize)]
pub struct Error {
    kind: ValueError,
    location: (usize, usize),
    line: String,
    path: Option<PathBuf>,
}

impl Error {
    pub fn with_path(mut self, path: &Path) -> Self {
        self.path = Some(path.to_path_buf());
        self
    }

    pub fn add_line(mut self, line: usize) -> Self {
        self.location.0 += line;
        self
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ValueError {
    #[cfg(feature = "eth")]
    #[error(transparent)]
    Zemen(#[from] zemen::error::Error),

    #[cfg(not(feature = "eth"))]
    #[error("feature `eth` is not enabled")]
    EthNotEnabled,

    #[error(transparent)]
    Date(#[from] time::error::ComponentRange),

    #[error("can not parse input")]
    Parse(String),
}

impl serde::Serialize for ValueError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match &self.kind {
            #[cfg(feature = "eth")]
            ValueError::Zemen(_) => "invalid ethiopian date",

            #[cfg(not(feature = "eth"))]
            ValueError::EthNotEnabled => "feature `eth` is not enabled",

            ValueError::Date(_) => "invalid gregorian date",
            ValueError::Parse(msg) => msg.as_str(),
        };

        let path = self
            .path
            .as_ref()
            .map(|p| p.to_string_lossy())
            .unwrap_or_default();

        let spacing = " ".repeat(self.location.0.to_string().len());

        write!(
            f,
            "error: {}\n  --> {}:{}:{}\n{spacing}  |\n {} | {} \n{spacing}  |\n  = {}\n\n",
            self.kind,
            path,
            self.location.0,
            self.location.1,
            self.location.0,
            self.line.trim(),
            msg
        )
    }
}

pub fn qerror(e: pest::error::Error<Rule>) -> Error {
    let line = e.line().to_string();

    let location = match &e.line_col {
        pest::error::LineColLocation::Pos(ref loc) => loc,
        pest::error::LineColLocation::Span(_, ref loc) => loc,
    };

    Error {
        location: (location.0, location.1),
        kind: ValueError::Parse(e.variant.message().to_string()),
        line,
        path: None,
    }
}

pub fn invalid_format<E: Into<ValueError>>(
    location: (usize, usize),
    line: String,
) -> impl FnOnce(E) -> Error {
    move |e: E| Error {
        kind: e.into(),
        location,
        line,
        path: None,
    }
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
