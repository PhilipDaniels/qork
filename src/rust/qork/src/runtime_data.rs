use fs::{DataDir, BaseDir};

use configuration::Configuration;
use execution_timer::ExecutionTimer;
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

const MRU_FILE : &'static str = "mru.toml";

impl RuntimeData {
    /// Constructs a new RuntimeData object based on the default configuration.
    pub fn new(config: &Configuration) -> RuntimeData {
        RuntimeData {
            mru: MRUList::new(config.max_mru_items()),
            mru2: DataItem { filename: "mru.toml", data: MRUList::new(config.max_mru_items()) }
        }
    }

    pub fn load(config: &Configuration, data_dir: &DataDir) -> RuntimeData {
        let _timer = ExecutionTimer::with_start_message("RuntimeData::load");

        let mut rd = RuntimeData::new(&config);

        info!("Loading runtime data from data_directory {:?}", data_dir.home());

        data_dir.get_existing_path(MRU_FILE)
            .map(|path| MRUList::load(config.max_mru_items(), &path)
                .map(|mru| {
                    rd.mru = mru;
                    info!("Loaded {} items into the MRU List from {:?}", rd.mru.iter().count(), path);
                }));

        rd
    }

    pub fn save(&mut self, data_dir: &DataDir) {
        let _timer = ExecutionTimer::with_start_message("RuntimeData::save");

        if self.mru.is_changed() {
            data_dir.get_proposed_path(MRU_FILE)
                .map(|path| self.mru.save(&path)
                    .map(|num_bytes| info!("Wrote {} bytes to {:?}", num_bytes, &path)));
        }
    }

    pub fn mru(&mut self) -> &mut MRUList {
        &mut self.mru
    }
}
