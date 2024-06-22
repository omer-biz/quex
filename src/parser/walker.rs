use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{error::QuexIoError, parser::Schedule};

fn walk_dir(path: &PathBuf) -> Result<Vec<Schedule>, QuexIoError> {
    if path.is_dir() {
        let mut schedules: Vec<Schedule> = vec![];

        // generic io failure
        for entry in path.read_dir().map_err(QuexIoError::new(path.clone()))? {
            // generic io failure
            let ent = entry.map_err(QuexIoError::new(path.clone()))?.path();

            let sch = walk_dir(&ent);
            match sch {
                Ok(sch) => schedules.extend(sch),
                // other errors will be reported here
                Err(e) => println!("Error: {e:#?}"),
            }
        }
        Ok(schedules)
    } else {
        if path.extension().and_then(OsStr::to_str).unwrap_or("") != "md" {
            return Ok(vec![]);
        };

        let file = File::open(path).map_err(QuexIoError::new(path.clone()))?; // unable to open file

        let mut quex_block = 0;

        let mut raw_quex = String::new();
        let reader = BufReader::new(file);
        let mut line_iter = reader.lines();
        let mut schedules = vec![];

        loop {
            let Some(line) = line_iter.next() else {
                break Ok(schedules);
            };
            let line = line.map_err(QuexIoError::new(path.clone()))?; // file read error ? maybe generic

            if line == "```quex" {
                quex_block += 1;

                for line in line_iter.by_ref() {
                    let mut line = line.map_err(QuexIoError::new(path.clone()))?; // file read error ? maybe generic

                    // TODO: what if there was EOF before the end of the `quex` block?
                    if line == "```" {
                        break;
                    }
                    line.push('\n');

                    // TODO: creates a redundant of the file contents one for `raw_quex` and one
                    // for `line` it's self, since the lines from "```quex" to "```" are
                    // consecutive one allocation of String would have been enough. Find a way to
                    // do that. maybe with unsafe
                    raw_quex.push_str(&line);
                }
            }

            if !raw_quex.is_empty() {
                let schedule = super::parse_quex(&raw_quex);
                match schedule {
                    Ok(schedule) => schedules.extend(schedule),
                    Err(e) => {
                        // parsing errors will be printed here
                        println!(
                            "Error in file: {}\nblock: {quex_block}\n{e:#?}",
                            path.display()
                        );
                    }
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
        let path = PathBuf::from("/home/omer/Documents/Notes/Rust/quex/test_quex_files/");

        let schedules = super::walk_dir(&path);
        println!("{:#?}", schedules);
    }
}
