use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{
    error::{io, Error, ValueError},
    parser::{self, Schedule},
};

pub type Schedules = Vec<Schedule>;
pub type QErrors = Vec<Error>;

pub fn walk_dir(path: PathBuf) -> Result<(Schedules, QErrors), io::FileError> {
    if path.is_dir() {
        let mut schedules: Schedules = vec![];
        let mut errors = vec![];

        let entries = match path.read_dir() {
            Ok(entries) => entries,
            Err(e) => return Err(io::FileError::new(path, e)),
        };

        // generic io failure
        for entry in entries {
            // generic io failure
            let ent = match entry {
                Ok(e) => e.path(),
                Err(e) => return Err(io::FileError::new(path, e)),
            };

            let (schs, errs) = walk_dir(ent)?;

            schedules.extend(schs);
            errors.extend(errs);
        }
        Ok((schedules, errors))
    } else {
        let file_extension = path.extension().and_then(OsStr::to_str).unwrap_or("");

        if file_extension == "md" {
            let file = match File::open(&path) {
                Ok(file) => file,
                Err(e) => return Err(io::FileError::new(path, e)),
            };

            let reader = BufReader::new(file);
            let mut line_iter = reader.lines().enumerate();
            let mut schedules: Schedules = vec![];
            let mut errors = Error::new(path.clone());

            loop {
                let Some((_, line)) = line_iter.next() else {
                    let rec_is_hard = if errors.is_empty() {
                        Vec::with_capacity(0)
                    } else {
                        vec![errors]
                    };

                    break Ok((schedules, rec_is_hard));
                };

                let line = match line {
                    Ok(line) => line,
                    Err(e) => return Err(io::FileError::new(path, e)),
                }; // file read error ? maybe generic

                if line.trim().contains("```quex") {
                    for (line_number, line) in line_iter.by_ref() {
                        let line = match line {
                            Ok(line) => line,
                            // TODO: Handle EOF
                            Err(e) => return Err(io::FileError::new(path, e)),
                        };

                        if line.trim().contains("```") {
                            break;
                        }

                        match parser::parse_line(line.as_str()) {
                            Ok(event) => schedules.push(event),
                            Err(e) => errors.push(ValueError::new(e, line_number + 1, line)),
                        };
                    }
                }
            }
        } else if file_extension == "quex" {
            let file = match File::open(&path) {
                Ok(file) => file,
                Err(e) => return Err(io::FileError::new(path, e)),
            };

            let reader = BufReader::new(file);
            let mut schedules = vec![];
            let mut errors = Error::new(path.clone());

            for (line_number, line) in reader.lines().enumerate() {
                let line = match line {
                    Ok(line) => line,
                    Err(e) => return Err(io::FileError::new(path, e)),
                };

                if line.is_empty() {
                    continue;
                }

                match parser::parse_line(line.as_str()) {
                    Ok(event) => schedules.push(event),
                    Err(e) => errors.push(ValueError::new(e, line_number + 1, line)),
                }
            }

            let rec_is_hard = if errors.is_empty() {
                Vec::with_capacity(0)
            } else {
                vec![errors]
            };

            Ok((schedules, rec_is_hard))
        } else {
            return Ok((vec![], vec![]));
        }
    }
}
