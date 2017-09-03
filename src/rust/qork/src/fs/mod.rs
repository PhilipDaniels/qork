use std::env::temp_dir;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use rand::{thread_rng, Rng};

mod config_dir;
mod data_dir;
mod base_dir;
mod runtime_dir;

pub use fs::base_dir::BaseDir;
pub use fs::config_dir::ConfigDir;
pub use fs::data_dir::DataDir;
pub use fs::runtime_dir::RuntimeDir;

// TODO: load_file_as_string

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

/// Generate a filename that, at the time of the call, does not exist. This is mainly
/// intended for use in testing scenarios - to check how functions behave when passed
/// non-existing filenames - not in real production code, because it exposes a
/// TOCTOU window. The filename will be under the temp directory.
pub fn filename_that_does_not_exist() -> PathBuf {
    let mut p = temp_dir();

    for _ in 0..5 {
        let part : String = thread_rng().gen_ascii_chars().take(5).collect();
        let part = part.to_lowercase();
        p.push(part);
    }

    let starting_p = p.clone();

    // Keep generating random strings to put on the end until we get something
    // that does not exist. Do not make the path infinitely deep though.
    loop {
        if !p.exists() {
            break;
        }

        let part : String = thread_rng().gen_ascii_chars().take(5).collect();
        let part = part.to_lowercase();
        let mut p = starting_p.clone();
        p.push(part);
    }

    p
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filename_that_does_not_exist_returns_filename_that_does_not_exist_in_temp_dir() {
        let p = filename_that_does_not_exist();
        assert!(!p.exists());
        assert!(p.starts_with(temp_dir()));
        println!("{}", p.to_str().unwrap());
    }
}
