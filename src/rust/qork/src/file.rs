use std::fs::File;
use std::path::PathBuf;
//use std::io::{self, Read, BufReader};
use std::io::{BufReader};
use std::io::prelude::*;

pub fn load_file_as_vector(filename: &PathBuf) -> Result<Vec<String>, String> {
    File::open(filename)
        .map_err(|err| err.to_string())
        .and_then(|mut f| {
            let mut v = Vec::<String>::new();
            let mut f = BufReader::new(f);

            for line in f.lines() {
                match line {
                    Ok(line) => { v.push(line) },
                    Err(e) => return Err(e.to_string())
                }
            }

            Ok(v)
        })
}

// TODO: load_file_as_string
