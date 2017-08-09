use std::fs::File;
use std::io::{Read};
use toml;
use context::Context;
use execution_timer::ExecutionTimer;

// Stores the configuration. Will be read from config.toml. Any values not
// present in the file will be defaulted using the 'default' method below.
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Configuration {
    max_mru_items: u32,
    max_open_files: u32
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            max_mru_items: 20,
            max_open_files: 200
        }
    }
}

const CONFIG_FILE : &'static str = "config.toml";

impl Configuration {
    pub fn load_user_configuration(context: &Context) -> Configuration {
        let _timer = ExecutionTimer::with_start_message("load_user_configuration");

        if !context.program_info().parsed_args().load_config() {
            info!("Loading of user configuration is disabled by command line option.");
            return Configuration::default();
        }

        let xdg = context.xdg();
        let dir = xdg.get_config_home();
        if !dir.exists() {
            warn!("The config_directory does not exist, no config will be loaded, config_directory: {:?}", dir);
            return Configuration::default();
        }

        if !dir.is_dir() {
            warn!("The config_directory is a file, not a directory, no config will be loaded, config_directory: {:?}", dir);
            return Configuration::default();
        }

        info!("Loading user configuration file {:?} from config_directory: {:?}", CONFIG_FILE, dir);

        let path = xdg.place_config_file(CONFIG_FILE);
        if path.is_err() {
            warn!("Could not locate {} file in xdg directory structure", CONFIG_FILE);
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

        // Ok, the file exists and we can try to load it.
        let cfg = File::open(&path)
            .map_err(|err| err.to_string())
            .and_then(|mut f| {
                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .map_err(|err| err.to_string())
                    .map(|num_bytes_read| {
                        info!("Read {} bytes from {:?}", num_bytes_read, &path);
                        contents
                    })
            })
            .and_then(|contents| {
                toml::from_str::<Configuration>(&contents)
                    .map_err(|err| err.to_string())
            })
            .map_err(|err| warn!("Error reading {:?}: {:?}", CONFIG_FILE, err))
            .unwrap_or_default();

        info!("{:?}", cfg);

        cfg
    }
}
