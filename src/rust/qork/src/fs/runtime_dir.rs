use fs::BaseDir;
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;

/// Based on the XDG Base Directory Specification.
/// `RuntimeDir` is a wrapper for "There is a single base directory relative to which user-specific
/// runtime files and other file objects should be placed. This directory is defined by the
/// environment variable `$XDG_RUNTIME_DIR`.". `RuntimeDir` is implemented as a wrapper around the
/// xdg *runtime* functions.
///
/// This directory is where things such as temporary pipes and sockets will be
/// stored.
pub struct RuntimeDir {
    xdg: BaseDirectories,
    is_valid: bool,
}

impl BaseDir for RuntimeDir {
    fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Gets the root directory.
    fn home(&self) -> PathBuf {
        // self.xdg.get_runtime_home()
        unimplemented!()
    }

    /// Gets a filepath within the root directory, creating leading
    /// directories. Logs and returns None if an error occurs.
    fn get_proposed_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        if !self.is_valid {
            return None;
        }

        match self.xdg.place_runtime_file(&path) {
            Err(e) => {
                error!("Error attempting to place file {:?}, err = {}. Returning None.", &path.as_ref(), e);
                None
            }
            Ok(p) => Some(p),
        }
    }


    /// Get the path of an existing file, or return None if the file does not
    /// exist or the path refers to a directory.
    fn get_existing_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        if !self.is_valid {
            return None;
        }

        self.xdg
            .find_runtime_file(path)
            .and_then(|p| if p.is_dir() {
                          error!("The path {:?} is a directory, expected a file. Returning None.", p);
                          None
                      } else {
                          Some(p)
                      })
    }
}

impl RuntimeDir {
    pub fn new(xdg: BaseDirectories) -> RuntimeDir {
        let is_valid = xdg.has_runtime_directory();

        RuntimeDir { xdg, is_valid }
    }
}
