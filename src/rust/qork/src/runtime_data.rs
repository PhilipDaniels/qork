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
    mru: MRUList,
    mru2: DataItem<MRUList>
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
            warn!("Could not locate runtime data file {}  in xdg directory structure", filename);
            return None;
        }

        let path = path.unwrap();

        if path.exists() && !path.is_file() {
            warn!("The runtime data file {:?} is not a file", path);
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
            info!("MRU is changed, path = {:?}", path);

            if let Some(filename) = path {
                match self.mru.save(filename.as_path()) {
                    Ok(num_bytes) => { info!("Wrote {} bytes to {:?}", num_bytes, filename); },
                    Err(e) => { warn!("Could not save MRU List to {:?}", filename); }
                }
            }
        }
    }

    pub fn mru(&mut self) -> &mut MRUList {
        &mut self.mru
    }
}




const MRU_FILE : &'static str = "mru.toml";

fn load_mru(max_mru_items: usize, filename: &PathBuf) -> Option<MRUList> {
    let list = file::load_to_vector(filename);

    // TODO: if num lines loaded > max_mru_items then the mru list needs to be marked as changed.

    match list {
        Ok(list) => {
            info!("Loaded {} lines from {:?}", list.len(), filename);
            let mru = MRUList::from_slice(max_mru_items, &list);
            dump(&mru);
            Some(mru)
        },
        Err(e) => None
    }
}

pub fn dump(mru: &MRUList) {
    for (i, file) in mru.iter().enumerate() {
        info!("{} = {:?}", i, file);
    }
}
