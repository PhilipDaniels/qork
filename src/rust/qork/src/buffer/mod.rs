use fs;
use std::path::{Path, PathBuf};
use std::fmt;
use time::{Tm, now_utc};
use xi_rope::Rope;

mod buffer_collection;

pub use buffer::buffer_collection::BufferCollection;

/// A `Buffer` represents the in-process data structures of an open file. This includes the buffer
/// contents and certain tracking information to support editing operations. It does not include
/// things to do with display: a file can be opened in a `Buffer` without being currently displayed,
/// in fact it need never be displayed at all. On the other hand, a `Buffer` may be displayed in
/// several different `Windows` simultaneously.
pub struct Buffer {
    id: i64,

    /// If this Buffer corresponds to a file, the name of the file.
    filename: Option<PathBuf>,

    /// The buffer's nominal title.
    title: String,

    /// The data in the buffer, expressed as a Rope structure.
    data: Rope,

    /// Whether the buffer is changed.
    is_changed: bool,

    /// The time that the buffer was created. This is NOT the same as the file creation
    /// time (indeed, there might not even be a file).
    created_time_utc: Tm,

    /// The time that the buffer was last accessed. This is NOT necessarily the same as
    /// the time that the buffer was last changed.
    last_accessed_time_utc: Tm,

    /// The time that the buffer was last changed.
    last_changed_time_utc: Tm
}

impl Buffer {
    pub fn new() -> Buffer {
        let now = now_utc();

        Buffer {
            id: 0,
            filename: None,
            title: String::default(),
            data: Rope::from(""),
            is_changed: false,
            created_time_utc: now,
            last_accessed_time_utc: now,
            last_changed_time_utc: now
        }
    }

    pub fn open_file<P: AsRef<Path>>(filename: P) -> Option<Buffer> {
        let filename = filename.as_ref();
        let now = now_utc();

        match fs::load_to_string(filename) {
            Ok(contents) => {
                Some(Buffer {
                    id: 0,
                    filename: Some(PathBuf::from(filename)),
                    title: String::from(filename.to_str().unwrap()),
                    data: Rope::from(contents),
                    is_changed: false,
                    created_time_utc: now,
                    last_accessed_time_utc: now,
                    last_changed_time_utc: now
                })
            },
            Err(e) => { warn!("{}", e); None }
        }
    }

    pub fn set_changed(&mut self) {
        self.is_changed = true;
        self.last_changed_time_utc = now_utc();
    }

    pub fn set_accessed(&mut self) {
        self.last_accessed_time_utc = now_utc();
    }

    pub fn is_changed(&self) -> bool {
        self.is_changed
    }

    pub fn created_time_utc(&self) -> &Tm {
        &self.created_time_utc
    }

    pub fn last_accessed_time_utc(&self) -> &Tm {
        &self.last_accessed_time_utc
    }

    pub fn last_changed_time_utc(&self) -> &Tm {
        &self.last_changed_time_utc
    }
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Buffer) -> bool {
        self.id == other.id
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer {{ id: {}, filename: \"{:?}\", created_time_utc: {:?} }}",
            self.id, self.filename, self.created_time_utc)
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;

    /*
    #[test]
    fn set_changed_sets_changed_flag_and_changed_time() {
        let mut b = Buffer::new();
        b.set_changed();
        assert!(b.is_changed());
        assert!(b.last_changed_time_utc() > b.created_time_utc());
    }
    */
}

