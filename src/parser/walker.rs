use std::{ffi::OsStr, fs::File, io::BufRead, path::PathBuf};

use super::Schedule;

fn walk_dir(path: PathBuf) -> Vec<Schedule> {
    if path.is_dir() {
        let mut schedules = vec![];
        for entry in path.read_dir().unwrap() {
            schedules.extend(walk_dir(entry.unwrap().path()));
        }
        schedules
    } else {
        if path.extension().and_then(OsStr::to_str).unwrap() != "md" {
            return vec![];
        };

        let file = File::open(path).unwrap();
        // let mut line_number = 0;

        let mut raw_quex = String::new();
        let reader = std::io::BufReader::new(file);
        let mut line_iter = reader.lines();
        let mut schedules = vec![];

        loop {
            let Some(line) = line_iter.next() else {
                break schedules;
            };
            let line = line.unwrap();

            if line == "```quex" {
                while let Some(line) = line_iter.next() {
                    let mut line = line.unwrap();
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
                schedules.extend(super::parse_quex(&raw_quex).unwrap());
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

        let schedules = super::walk_dir(path);
        println!("{:#?}", schedules);
    }
}
