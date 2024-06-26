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

type Schedules = Vec<Schedule>;
type QErrors = Vec<Error>;

fn walk_dir(path: &PathBuf) -> Result<(Schedules, QErrors), io::Error> {
    if path.is_dir() {
        let mut schedules: Schedules = vec![];
        let mut errors = vec![];

        // generic io failure
        for entry in path.read_dir().map_err(io::Error::new(path.clone()))? {
            // generic io failure
            let ent = entry.map_err(io::Error::new(path.clone()))?.path();
            let (schs, errs) = walk_dir(&ent)?;

            schedules.extend(schs);
            errors.extend(errs);
            // match sch {
            //     Ok(sch) => schedules.extend(sch),
            //     // other errors will be reported here
            //     Err(e) => println!("Error: {e:#?}"),
            // }
        }
        Ok((schedules, errors))
    } else {
        if path.extension().and_then(OsStr::to_str).unwrap_or("") != "md" {
            // TODO: error `file` is not a markdown file
            return Ok((vec![], vec![]));
        };
        println!("Processing file: {}", path.display());

        let file = File::open(path).map_err(io::Error::new(path.clone()))?; // unable to open file

        let mut raw_quex = String::new();
        let reader = BufReader::new(file);
        let mut line_iter = reader.lines().enumerate();
        let mut schedules: Schedules = vec![];
        let mut errors = vec![];

        loop {
            let Some((line_num, line)) = line_iter.next() else {
                break Ok((schedules, errors));
            };
            let line = line.map_err(io::Error::new(path.clone()))?; // file read error ? maybe generic

            if line == "```quex" {
                for (_, line) in line_iter.by_ref() {
                    let mut line = line.map_err(io::Error::new(path.clone()))?; // file read error ? maybe generic

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
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_walk_dir() {
        let path = PathBuf::from("test_quex_files/");

        let (schedules, errors) = super::walk_dir(&path).unwrap();
        println!("{:#?}", schedules);

        // TODO: pest implementation is leaking, fix the error messages.
        for err in errors {
            eprintln!("{err}");
        }
    }
}
