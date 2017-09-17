use std::path::PathBuf;
use std::fmt;
use time::{Tm, now_utc};
use xi_rope::Rope;

mod buffer_collection;
mod buffer_factory;

pub use buffer::buffer_collection::BufferCollection;
pub use buffer::buffer_factory::BufferFactory;

/// A `Buffer` represents the in-process data structures of an open file. This includes the buffer
/// contents and certain tracking information to support editing operations. It does not include
/// things to do with display: a file can be opened in a `Buffer` without being currently displayed,
/// in fact it need never be displayed at all. On the other hand, a `Buffer` may be displayed in
/// several different `Windows` simultaneously.
pub struct Buffer {
    id: u64,

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
    last_accessed_time_utc: Tm,
    last_changed_time_utc: Tm
}

impl Buffer {
    pub fn id(&self) -> u64 {
        self.id
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
    use time::now_utc;

    #[test]
    fn set_changed_sets_changed_flag_and_changed_time() {
        let mut fac = BufferFactory::new();
        let mut b = fac.new_empty_buffer();
        b.set_changed();
        assert!(b.is_changed());
        assert!(b.last_changed_time_utc() > b.created_time_utc());
    }
}
