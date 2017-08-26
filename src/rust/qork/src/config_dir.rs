use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;

pub struct ConfigDir {
    xdg: BaseDirectories
}

pub struct DataDir {
    xdg: BaseDirectories
}

impl ConfigDir {
    pub fn new(xdg: BaseDirectories) -> ConfigDir {
        // TODO: Check that the home dir exists and is a directory.
        // TODO: Use a command line flag to turn every function into a no-op.
        // TODO: Move these methods to a trait and use them to implement it on both classes.
        // TODO: Allow "no-op" operations under the control of a flag?
        // TODO: xdg:  create_{config,data,cache,runtime}_directory  - creates dirs under the XDG dir structure
        // TODO: xdg:  list_{config,data,cache,runtime}_files_[once] - lists files under the XDG dir structure
        ConfigDir { xdg }
    }

    /// Gets the root configuration directory.
    /// xdg equiv: get_{config,data,cache}_home
    pub fn home(&self) -> PathBuf {
        self.xdg.get_config_home()
    }

    /// Gets a filepath within the config directory, creating leading directories. Logs and returns None if an error occurs.
    /// xdg equiv: place_{config,data,cache,runtime}_file -> ioResult<PathBuf>
    pub fn get_proposed_path<P>(&self, path: P) -> Option<PathBuf>
        where P: AsRef<Path>
    {
        match self.xdg.place_config_file(&path) {
            Err(e) => { error!("Error attempting to place file {:?}, err = {}. Returning None.", &path.as_ref(), e); None },
            Ok(p) => Some(p)
        }
    }

    /// Get the path of an existing file, or return None if the file does not exist or the path refers to a directory.
    /// xdg equiv: find_{config,data,cache,runtime}_file -> Option<PathBuf>
    pub fn get_existing_path<P>(&self, path: P) -> Option<PathBuf>
        where P: AsRef<Path>
    {
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
    pub fn open<P>(&self, path: P) -> Option<File>
        where P: AsRef<Path>
    {
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
    pub fn create<P>(&self, path: P) -> Option<File>
        where P: AsRef<Path>
    {
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
    pub fn open_with_options<P>(&self, path: P, options: &OpenOptions) -> Option<File>
        where P: AsRef<Path>
    {
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
