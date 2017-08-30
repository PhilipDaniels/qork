use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::Path;

mod config_dir;
mod data_dir;
mod base_dir;
mod runtime_dir;

pub use fs::base_dir::BaseDir;
pub use fs::config_dir::ConfigDir;
pub use fs::data_dir::DataDir;
pub use fs::runtime_dir::RuntimeDir;

pub fn load_to_vector(filename: &Path) -> Result<Vec<String>, String> {
    File::open(filename)
        .map_err(|err| err.to_string())
        .and_then(|f| {
            let mut v = Vec::<String>::new();
            let f = BufReader::new(f);

            for line in f.lines() {
                match line {
                    Ok(line) => v.push(line),
                    Err(e) => return Err(e.to_string()),
                }
            }

            Ok(v)
        })
}

pub fn save_from_vector(filename: &Path, data: Vec<String>) -> Result<usize, String> {
    File::create(filename)
        .map_err(|err| err.to_string())
        .and_then(|f| {
            let mut f = BufWriter::new(f);
            let mut byte_count = 0;
            for line in data {
                let bytes = line.as_bytes();
                match f.write(bytes) {
                    Ok(_) => {}
                    Err(e) => return Err(e.to_string()),
                }

                match f.write(b"\n") {
                    Ok(_) => {}
                    Err(e) => return Err(e.to_string()),
                }

                byte_count += bytes.len() + 1;
            }

            Ok(byte_count)
        })
}

// TODO: load_file_as_string
// TODO: filename that does not exist.
