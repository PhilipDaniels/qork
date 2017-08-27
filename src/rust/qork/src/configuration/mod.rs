use std::io::prelude::*;
use toml;
use execution_timer::ExecutionTimer;
use fs::{BaseDir, ConfigDir};

// Stores the configuration. Will be read from config.toml. Any values not
// present in the file will be defaulted using the 'default' method below.
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Configuration {
    max_mru_items: usize,
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            max_mru_items: 20
        }
    }
}

const CONFIG_FILE : &'static str = "config.toml";

impl Configuration {
    pub fn max_mru_items(&self) -> usize {
        self.max_mru_items
    }

    pub fn load_user_configuration(cd: &ConfigDir) -> Configuration {
        let _timer = ExecutionTimer::with_start_message("load_user_configuration");

        let mut cfg = Configuration::default();

        if let Some((mut f, p)) = cd.open(CONFIG_FILE) {
            cfg = {
                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .map_err(|err| err.to_string())
                    .map(|num_bytes_read| {
                        info!("Read {} lines ({} bytes) from {:?}", contents.lines().count(), num_bytes_read,  p);
                        contents
                    })
                .and_then(|contents| {
                    toml::from_str::<Configuration>(&contents)
                        .map_err(|err| err.to_string())
                })
                .map_err(|err| warn!("Error reading {:?}: {:?}", CONFIG_FILE, err))
                .unwrap_or_default()
            };
        }

        info!("{:?}", cfg);

        cfg
    }
}
