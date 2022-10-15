use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file_path = std::env::args().nth(1).expect("no file given");
    let empty_line = Regex::new(r"^\s*$").unwrap();
    let mustache = Regex::new(r"^\s*}\s*$").unwrap();
    let buffered = BufReader::new(File::open(file_path)?);

    let mut new_line_found = false;
    let lines_buffer = buffered
        .lines();
    for (i, line_result) in lines_buffer.enumerate() {
        let line = &line_result?;
        if empty_line.is_match(line) && !new_line_found {
            new_line_found = true;
            continue;
        }

        if new_line_found {
            if empty_line.is_match(line) {
                println!("{}: double new line detected!", i);
            }
            else if mustache.is_match(line) {
                println!("{}: extra new line before {{ detected!", i);
            }
            new_line_found = false
        }
    }

    Ok(())
}

