use fs;
use std::path::{Path, PathBuf};
use xi_rope::Rope;

/// A `Buffer` represents the in-process data structures of an open file. This includes the buffer
/// contents and certain tracking information to support editing operations. It does not include
/// things to do with display: a file can be opened in a `Buffer` without being currently displayed,
/// in fact it need never be displayed at all. On the other hand, a `Buffer` may be displayed in
/// several different `Windows` simultaneously.
pub struct Buffer {
    /// If this Buffer corresponds to a file, the name of the file.
    filename: Option<PathBuf>,
    /// The data in the buffer, expressed as a Rope structure.
    data: Rope
}

impl Buffer {
    pub fn new_empty_buffer() -> Buffer {
        Buffer {
            filename: None,
            data: Rope::from("")
        }
    }

    pub fn open_file(filename: &Path) -> Option<Buffer> {
        match fs::load_to_string(filename) {
            Ok(contents) => {
                Some(Buffer {
                    filename: Some(PathBuf::from(filename)),
                    data: Rope::from(contents)
                })
            },
            Err(e) => { warn!("{}", e); None }
        }

    }

    // pub fn system_info(&self) -> &SystemInfo {
    //     &self.system_info
    // }
}