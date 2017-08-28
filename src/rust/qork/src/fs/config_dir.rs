use std::path::{Path, PathBuf};
use xdg::BaseDirectories;
use fs::BaseDir;

/// Based on the XDG Base Directory Specification.
/// ConfigDir is a wrapper for "There is a single base directory relative to which user-specific configuration files should be written.
/// This directory is defined by the environment variable $XDG_CONFIG_HOME.". ConfigDir is implemented as
/// a wrapper around the xdg *config* functions.
///
/// The data stored within this directory is configuration data, such as settings and flags.
/// It typically does not change during an invocation of Qork.
pub struct ConfigDir {
    xdg: BaseDirectories,
    is_valid: bool
}

impl BaseDir for ConfigDir {
    fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Gets the root directory.
    fn home(&self) -> PathBuf {
        self.xdg.get_config_home()
    }

    /// Gets a filepath within the root directory, creating leading directories. Logs and returns None if an error occurs.
    fn get_proposed_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>
    {
        if !self.is_valid { return None; }

        match self.xdg.place_config_file(&path) {
            Err(e) => { error!("Error attempting to place file {:?}, err = {}. Returning None.", &path.as_ref(), e); None },
            Ok(p) => Some(p)
        }
    }

    /// Get the path of an existing file, or return None if the file does not exist or the path refers to a directory.
    fn get_existing_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>
    {
        if !self.is_valid { return None; }

        self.xdg.find_config_file(path)
            .and_then(|p| if p.is_dir() {
                error!("The path {:?} is a directory, expected a file. Returning None.", p);
                None
            }
            else {
                Some(p)
            })
    }
}

impl ConfigDir {
    pub fn new(xdg: BaseDirectories, load_config: bool) -> ConfigDir {
        let mut is_valid = load_config;
        if !load_config {
            info!("Loading and saving of user configuration is disabled by command line option.");
        } else {
            let home = xdg.get_config_home();
            if !home.is_dir() {
                // Print to stderr, because this scenario means logging probably did not get configured.
                // Likewise, logging will not be configured if the directory does not exist, so there is no point
                // logging anything in that scenario.
                eprintln!("The root configuration directory {:?} is not a directory. No config will be loaded or saved.", home);
                is_valid = false;
            }
        }

        ConfigDir {
            xdg, is_valid
        }
    }
}
