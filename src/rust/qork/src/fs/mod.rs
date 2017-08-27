use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;
use std::io::{Read, BufReader, BufWriter};
use std::io::prelude::*;
use toml;
use context::Context;
use execution_timer::ExecutionTimer;

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
// TODO: filename that does not exist.
