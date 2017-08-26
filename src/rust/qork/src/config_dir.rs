use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;

pub struct ConfigDir {
    xdg: BaseDirectories,
    valid: bool
}

pub struct DataDir {
    xdg: BaseDirectories,
    valid: bool
}

pub trait WellKnownDir {
    // TODO: xdg:  create_{config,data,cache,runtime}_directory  - creates dirs under the XDG dir structure
    // TODO: xdg:  list_{config,data,cache,runtime}_files_[once] - lists files under the XDG dir structure

    /// Gets the root directory.
    /// xdg equiv: get_{config,data,cache}_home
    fn home(&self) -> PathBuf;

    /// Gets a filepath within the root directory, creating leading directories. Logs and returns None if an error occurs.
    /// xdg equiv: place_{config,data,cache,runtime}_file -> ioResult<PathBuf>
    fn get_proposed_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;

    /// Get the path of an existing file, or return None if the file does not exist or the path refers to a directory.
    /// xdg equiv: find_{config,data,cache,runtime}_file -> Option<PathBuf>
    fn get_existing_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;

    /// Opens a file in read-only mode, or returns None if the file cannot be opened.
    fn open<P: AsRef<Path>>(&self, path: P) -> Option<File>;

    /// Opens a file in write-only mode, if the file already exists it will be truncated.
    /// Returns None if the file cannot be opened.
    fn create<P: AsRef<Path>>(&self, path: P) -> Option<File>;

    /// Opens a file with specific options, or returns None if the file cannot be opened.
    fn open_with_options<P: AsRef<Path>>(&self, path: P, options: &OpenOptions) -> Option<File>;
}

impl WellKnownDir for ConfigDir {
    /// Gets the root directory.
    fn home(&self) -> PathBuf {
        self.xdg.get_config_home()
    }

    /// Gets a filepath within the root directory, creating leading directories. Logs and returns None if an error occurs.
    fn get_proposed_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>
    {
        if !self.valid { return None; }

        match self.xdg.place_config_file(&path) {
            Err(e) => { error!("Error attempting to place file {:?}, err = {}. Returning None.", &path.as_ref(), e); None },
            Ok(p) => Some(p)
        }
    }


    /// Get the path of an existing file, or return None if the file does not exist or the path refers to a directory.
    fn get_existing_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>
    {
        if !self.valid { return None; }

        self.xdg.find_config_file(path)
            .and_then(|p| if p.is_dir() {
                error!("The path {:?} is a directory, expected a file. Returning None.", p);
                None
            }
            else {
                Some(p)
            })
    }

    /// Opens a file in read-only mode, or returns None if the file cannot be opened.
    fn open<P: AsRef<Path>>(&self, path: P) -> Option<File>
    {
        if !self.valid { return None; }

        match self.get_existing_path(path) {
            None => None,
            Some(p) => {
                match File::open(&p) {
                    Err(e) => { error!("Error attempting to open file {:?}, err = {}", p, e); None },
                    Ok(f) => Some(f)
                }
            }
        }
    }

    /// Opens a file in write-only mode, if the file already exists it will be truncated.
    /// Returns None if the file cannot be opened.
    fn create<P: AsRef<Path>>(&self, path: P) -> Option<File>
    {
        if !self.valid { return None; }

        match self.get_proposed_path(path) {
            None => None,
            Some(p) => {
                match File::create(&p) {
                    Err(e) => { error!("Error attempting to create file {:?}, err = {}", p, e); None },
                    Ok(f) => Some(f)
                }
            }
        }
    }

    /// Opens a file with specific options, or returns None if the file cannot be opened.
    fn open_with_options<P: AsRef<Path>>(&self, path: P, options: &OpenOptions) -> Option<File>
    {
        if !self.valid { return None; }

        match self.get_proposed_path(path) {
            None => None,
            Some(p) => {
                match options.open(&p) {
                    Err(e) => { error!("Error attempting to open file {:?}, err = {}", p, e); None },
                    Ok(f) => Some(f)
                }
            }
        }
    }
}

impl ConfigDir {
    pub fn new(xdg: BaseDirectories, load_config: bool) -> ConfigDir {
        let mut valid = load_config;
        if !load_config {
            info!("Loading and saving of user configuration is disabled by command line option.");
        } else {
            let home = xdg.get_config_home();
            if !home.is_dir() {
                // Print to stderr, because this scenario means logging probably did not get configured.
                // Likewise, logging will not be configured if the directory does not exist, so there is no point
                // logging anything in that scenario.
                eprintln!("The root configuration directory {:?} is not a directory. No config will be loaded.", home);
                valid = false;
            }
        }

        ConfigDir {
            xdg, valid
        }
    }
}
