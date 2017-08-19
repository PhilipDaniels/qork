use std::fs::File;
use std::path::PathBuf;
use std::io::{self, Read, BufReader};
use std::io::prelude::*;
use toml;
use xdg::BaseDirectories;

use configuration::Configuration;
use execution_timer::ExecutionTimer;
use file;
use mru_list::MRUList;

/// Represents the persistent runtime data of the system. This is things like MRU lists
/// that we expect to get written to disk and be available the next time we start.
/// Therefore, it excludes things like buffer collections. Things that are virtually
/// always used are non-lazy, while things that are used less frequently are wrapped
/// by a Lazy<T>.

/// TODO: PersistentData or SessionData.
struct DataItem<T> {
    filename: &'static str,
    data: T
}

pub struct RuntimeData {
    mru: MRUList<String>,
    mru2: DataItem<MRUList<String>>
}

impl RuntimeData {
    /// Constructs a new RuntimeData object based on the default configuration.
    pub fn new(config: &Configuration) -> RuntimeData {
        RuntimeData {
            mru: MRUList::new(config.max_mru_items()),
            mru2: DataItem { filename: "mru.toml", data: MRUList::new(config.max_mru_items()) }
        }
    }

    fn place_file(xdg: &BaseDirectories, filename: &str) -> Option<PathBuf> {
        let path = xdg.place_data_file(filename);
        if path.is_err() {
            warn!("Could not locate {} file in xdg directory structure", filename);
            return None;
        }

        let path = path.unwrap();
        if !path.exists() {
            return None;
        }

        if !path.is_file() {
            warn!("The runtime data file {:?} appears to be a directory, no data will be loaded", path);
            return None;
        }

        Some(path)
    }

    pub fn load_runtime_data(config: &Configuration, xdg: &BaseDirectories) -> RuntimeData {
        let _timer = ExecutionTimer::with_start_message("load_runtime_data");

        let mut rd = RuntimeData::new(&config);

        let dir = xdg.get_data_home();
        if !dir.exists() {
            warn!("The data_directory does not exist, no runtime data will be loaded, data_directory: {:?}", dir);
            return rd;
        }

        if !dir.is_dir() {
            warn!("The data_directory is a file, not a directory, no runtime data will be loaded, data_directory: {:?}", dir);
            return rd;
        }

        info!("Loading runtime data from data_directory {:?}", dir);


        let path = RuntimeData::place_file(&xdg, MRU_FILE);
        if let Some(filename) = path {
            if let Some(mru) = load_mru(config.max_mru_items(), &filename) {
                rd.mru = mru;
            }
        }



        rd
    }

    /// Writes the runtime data to disk.
    pub fn save(&mut self, config: &Configuration, xdg: &BaseDirectories) {
        let _timer = ExecutionTimer::with_start_message("save_runtime_data");

        if self.mru.is_changed() {
            let path = RuntimeData::place_file(&xdg, MRU_FILE);
            if let Some(filename) = path {
                if let Ok(num_bytes) = save_mru(&filename, &self.mru) {
                    self.mru.clear_is_changed();
                    info!("Wrote {} bytes to {:?}", num_bytes, filename);
                }
            }
        }
    }

    pub fn mru(&mut self) -> &mut MRUList<String> {
        &mut self.mru
    }
}




const MRU_FILE : &'static str = "mru.toml";

fn load_mru(max_mru_items: usize, filename: &PathBuf) -> Option<MRUList<String>> {
    let list = file::load_to_vector(filename);

    match list {
        Ok(list) => {
            info!("Loaded {} lines from {:?}", list.len(), filename);
            Some(MRUList::clone_from_slice(max_mru_items, &list))
        },
        Err(e) => None
    }
}

fn save_mru(filename: &PathBuf, mru: &MRUList<String>) -> Result<usize, String> {
    let v = Vec::<String>::new();
    file::save_from_vector(filename, v)
}