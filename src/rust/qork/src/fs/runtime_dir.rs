use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;
use fs::BaseDir;

/// Based on the XDG Base Directory Specification.
/// RuntimeDir is a wrapper for "There is a single base directory relative to which user-specific runtime files and other file objects should be placed.
/// This directory is defined by the environment variable $XDG_RUNTIME_DIR. ". RuntimeDir is implemented as
/// a wrapper around the xdg *runtime* functions.
///
/// This directory is where things such as temporary pipes and sockets will be stored.
pub struct RuntimeDir {
    xdg: BaseDirectories,
    valid: bool
}

impl BaseDir for RuntimeDir {
    /// Gets the root directory.
    fn home(&self) -> PathBuf {
        self.xdg.get_runtime_home()
    }

    /// Gets a filepath within the root directory, creating leading directories. Logs and returns None if an error occurs.
    fn get_proposed_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>
    {
        if !self.valid { return None; }

        match self.xdg.place_runtime_file(&path) {
            Err(e) => { error!("Error attempting to place file {:?}, err = {}. Returning None.", &path.as_ref(), e); None },
            Ok(p) => Some(p)
        }
    }


    /// Get the path of an existing file, or return None if the file does not exist or the path refers to a directory.
    fn get_existing_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>
    {
        if !self.valid { return None; }

        self.xdg.find_runtime_file(path)
            .and_then(|p| if p.is_dir() {
                error!("The path {:?} is a directory, expected a file. Returning None.", p);
                None
            }
            else {
                Some(p)
            })
    }

    /// Opens a file in read-only mode, or returns None if the file cannot be opened.
    fn open<P: AsRef<Path>>(&self, path: P) -> Option<(File, PathBuf)>
    {
        if !self.valid { return None; }

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
        if !self.valid { return None; }

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
        if !self.valid { return None; }

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

impl RuntimeDir {
    pub fn new(xdg: BaseDirectories) -> RuntimeDir {
        let home = xdg.get_runtime_home();
        if !home.is_dir() {
            // Print to stderr, because this scenario means logging probably did not get configured.
            // Likewise, logging will not be configured if the directory does not exist, so there is no point
            // logging anything in that scenario.
            eprintln!("The root runtime directory {:?} is not a directory. Qork will probably not work correctly.", home);
            valid = false;
        }

        RuntimeDir {
            xdg, valid
        }
    }
}
