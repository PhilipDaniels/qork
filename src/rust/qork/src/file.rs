use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;

pub fn load_to_vector(filename: &Path) -> Result<Vec<String>, String> {
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

pub fn save_from_vector(filename: &Path, data: Vec<String>) -> Result<usize, String> {
    File::create(filename)
        .map_err(|err| err.to_string())
        .and_then(|mut f| {
            let mut f = BufWriter::new(f);
            let mut byte_count = 0;
            for line in data {
                let bytes = line.as_bytes();
                f.write(bytes);
                f.write(b"\n");
                byte_count += bytes.len() + 1;
            }

            Ok(byte_count)
        })
}

// TODO: load_file_as_string
