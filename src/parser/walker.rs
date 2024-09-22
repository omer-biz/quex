use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{
    error::{io, Error},
    parser::Schedule,
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

            let mut raw_quex = String::new();
            let reader = BufReader::new(file);
            let mut line_iter = reader.lines().enumerate();
            let mut schedules: Schedules = vec![];
            let mut errors = vec![];

            loop {
                let Some((line_num, line)) = line_iter.next() else {
                    break Ok((schedules, errors));
                };

                let line = match line {
                    Ok(line) => line,
                    Err(e) => return Err(io::FileError::new(path, e)),
                }; // file read error ? maybe generic

                if line == "```quex" {
                    for (_, line) in line_iter.by_ref() {
                        let mut line = match line {
                            Ok(line) => line,
                            Err(e) => return Err(io::FileError::new(path, e)),
                        }; // file read error ? maybe generic

                        // TODO: what if there was EOF before the end of the `quex` block?
                        if line == "```" {
                            break;
                        }
                        line.push('\n');

                        // TODO: creates a redundant of the file contents one for `raw_quex` and one
                        // for `line` it's self, since the lines from "```quex" to "```" are
                        // consecutive, one allocation of String would have been enough. Find a way to
                        // do that. maybe with unsafe
                        raw_quex.push_str(&line);
                    }
                }

                if !raw_quex.is_empty() {
                    let schedule = super::parse_quex(&raw_quex);
                    match schedule {
                        Ok(schedule) => schedules.extend(schedule),
                        Err(e) => errors.push(e.with_path(&path).add_line(line_num + 1)),
                    }

                    raw_quex.clear();
                }
            }
        } else if file_extension == "quex" {
            let file = match File::open(&path) {
                Ok(file) => file,
                Err(e) => return Err(io::FileError::new(path, e)),
            };

            // TODO: `read_to_string` is not efficient with big files
            // use buffered reading.
            let raw_quex = match std::io::read_to_string(file) {
                Ok(file) => file,
                Err(e) => return Err(io::FileError::new(path, e)),
            };

            // TODO: design issue from not continuing when parse error is found
            // this will change when we start to continue parsing even if we
            // encounter parse errors
            //
            // This may also hint to modulirizing calenders
            // see calender_modulrizing.md
            match super::parse_quex(&raw_quex) {
                Ok(s) => Ok((s, vec![])),
                Err(e) => Ok((vec![], vec![e.with_path(&path)])),
            }
        } else {
            return Ok((vec![], vec![]));
        }
    }
}
