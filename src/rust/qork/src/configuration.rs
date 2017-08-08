use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use toml;
use context::Context;
use execution_timer::ExecutionTimer;

// Stores the configuration. Will be read from config.toml. Any values not
// present in the file will be defaulted using the 'default' method below.
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Configuration {
    num_mru_items: u32
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            num_mru_items: 20
        }
    }
}

impl Configuration {
pub fn load_user_configuration(context: &Context) -> Configuration {
        let _timer = ExecutionTimer::with_start_message("main.load_user_configuration");

        if !context.program_info().parsed_args().load_config() {
            info!("Loading of user configuration is disabled.");
            return Configuration::default();
        }

        let xdg = context.xdg();
        let dir = xdg.get_config_home();
        if !dir.exists() {
            warn!("The config_directory does not exist, no config will be loaded, config_directory={:?}", dir);
            return Configuration::default();
        }

        if !dir.is_dir() {
            warn!("The config_directory is a file, not a directory, no config will be loaded, config_directory={:?}", dir);
            return Configuration::default();
        }

        info!("Loading user configuration from directory {:?}", dir);

        let path = xdg.place_config_file("config.toml");
        if path.is_err() {
            warn!("Could not locate config.toml file");
            return Configuration::default();
        }
        let path = path.unwrap();
        if !path.exists() {
            debug!("The file {:?} does not exist, no user config will be loaded", path);
            return Configuration::default();
        }
        if !path.is_file() {
            warn!("The user configuration file {:?} appears to be a directory, no user config will be loaded", path);
            return Configuration::default();
        }

        // Ok, the file exists and can be loaded.
        let file = File::open(&path);
        let mut toml = String::new();
        file.unwrap().read_to_string(&mut toml);
        let cfg : Configuration = toml::from_str(&toml).unwrap();
        info!("Got num_mru_items = {}", cfg.num_mru_items);

        cfg
    }
}
