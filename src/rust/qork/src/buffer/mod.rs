use fs;
use std::path::{Path, PathBuf};
use xi_rope::Rope;

mod buffer_collection;

pub use buffer::buffer_collection::BufferCollection;

/// A `Buffer` represents the in-process data structures of an open file. This includes the buffer
/// contents and certain tracking information to support editing operations. It does not include
/// things to do with display: a file can be opened in a `Buffer` without being currently displayed,
/// in fact it need never be displayed at all. On the other hand, a `Buffer` may be displayed in
/// several different `Windows` simultaneously.
pub struct Buffer {
    /// If this Buffer corresponds to a file, the name of the file.
    pub filename: Option<PathBuf>,
    /// The data in the buffer, expressed as a Rope structure.
    data: Rope,
    pub is_changed: bool
    // created_time_utc: Tm
    // last_accessed_time_utc: Tm
    // is_changed: bool
    // default_title: String   title to be used when there is only 1 view on a buffer. The filename
    //                         or Untitled<1>
    // TODO: Need title allocation function
}

impl Buffer {
    /// Creates a new empty buffer.
    pub fn new() -> Buffer {
        Buffer {
            filename: None,
            data: Rope::from(""),
            is_changed: false
        }
    }

    pub fn open_file<P: AsRef<Path>>(filename: P) -> Option<Buffer> {
        let filename = filename.as_ref();
        match fs::load_to_string(filename) {
            Ok(contents) => {
                Some(Buffer {
                    filename: Some(PathBuf::from(filename)),
                    data: Rope::from(contents),
                    is_changed: false
                })
            },
            Err(e) => { warn!("{}", e); None }
        }

    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;
}

