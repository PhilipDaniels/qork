use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

pub trait BaseDir {
    // TODO: xdg:  create_{config,data,cache,runtime}_directory  - creates dirs under the XDG dir structure
    // TODO: xdg:  list_{config,data,cache,runtime}_files_[once] - lists files under the XDG dir structure

    /// Checks whether the directory object is valid. If it is not valid,
    /// then all method calls will be no-ops.
    fn is_valid(&self) -> bool;

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
    fn open<P: AsRef<Path>>(&self, path: P) -> Option<(File, PathBuf)>
    {
        if !self.is_valid() { return None; }

        match self.get_existing_path(path) {
            None => None,
            Some(p) => {
                match File::open(&p) {
                    Err(e) => { error!("Error attempting to open file {:?}, err = {}", p, e); None },
                    Ok(f) => Some((f, p))
                }
            }
        }
    }

    /// Opens a file in write-only mode, if the file already exists it will be truncated.
    /// Returns None if the file cannot be opened.
    fn create<P: AsRef<Path>>(&self, path: P) -> Option<(File, PathBuf)>
    {
        if !self.is_valid() { return None; }

        match self.get_proposed_path(path) {
            None => None,
            Some(p) => {
                match File::create(&p) {
                    Err(e) => { error!("Error attempting to create file {:?}, err = {}", p, e); None },
                    Ok(f) => Some((f, p))
                }
            }
        }
    }

    /// Opens a file with specific options, or returns None if the file cannot be opened.
    fn open_with_options<P: AsRef<Path>>(&self, path: P, options: &OpenOptions) -> Option<(File, PathBuf)>
    {
        if !self.is_valid() { return None; }

        match self.get_proposed_path(path) {
            None => None,
            Some(p) => {
                match options.open(&p) {
                    Err(e) => { error!("Error attempting to open file {:?}, err = {}", p, e); None },
                    Ok(f) => Some((f, p))
                }
            }
        }
    }

}
