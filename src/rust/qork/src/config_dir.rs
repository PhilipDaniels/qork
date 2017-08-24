use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;

pub struct ConfigDir<'a> {
    xdg: &'a BaseDirectories
}

impl<'a> ConfigDir<'a> {
    pub fn new(xdg: &BaseDirectories) -> ConfigDir {
        ConfigDir { xdg }
    }

    pub fn home(&self) -> PathBuf {
        self.xdg.get_config_home()
    }

    /// Gets a filepath within the config directory, creating leading directories. Logs and returns None if an error occurs.
    pub fn place<P>(&self, path: P) -> Option<PathBuf>
        where P: AsRef<Path>
    {
        match self.xdg.place_config_file(&path) {
            Err(e) => { error!("Error attempting to place file {:?}, err = {}", &path.as_ref(), e); None },
            Ok(p) => Some(p)
        }
    }

    /// Get the path of an existing file, or return None if the file does not exist or the path refers to a directory.
    pub fn find<P>(&self, path: P) -> Option<PathBuf>
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
        match self.find(path) {
            None => None,
            Some(p) => {
                match File::open(&p) {
                    Err(e) => { error!("Error attempting to open file {:?}, err = {}", p, e); None },
                    Ok(f) => Some(f)
                }
            }
        }
    }

    // pub fn open_options<P>(&self, path: P, options: OpenOptions) -> Option<File>
    //     where P: AsRef<Path>
    // {
    //     let p = self.find(path).unwrap();
    //     options.open(p).ok()
    // }


    //pub fn create - opens in write-only mode
    //Move these methods to a trait and use them to implement it on both classes.
    //Allow "no-op" operations under the control of a flag?
}

pub struct DataDir {

}

/*
XDG supports these operations
  find_{config,data,cache,runtime}_file -> Option<PathBuf>       - gets an existing file, or returns None
  place_{config,data,cache,runtime}_file -> ioResult<PathBuf>    -

  create_{config,data,cache,runtime}_directory  - creates dirs under the XDG dir structure
  list_{config,data,cache,runtime}_files_[once] - lists files under the XDG dir structure
  get_{config,data,cache}_home                  - gets the root dir under the XDG dir structure

>> To load a file, use find(). We should wrap find() and check exists and is a file. Or we can write a series of open() methods.
>> To save a file, use place().

*/