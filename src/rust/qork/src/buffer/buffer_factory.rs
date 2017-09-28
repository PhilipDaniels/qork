use fs;
use std::path::{Path, PathBuf};
use time::now_utc;
use xi_rope::Rope;
use super::{Buffer, BufferId};

pub struct BufferFactory {
    next_buffer_id: BufferId
}

impl BufferFactory {
    pub fn new() -> BufferFactory {
        BufferFactory { next_buffer_id: 0 }
    }

    fn empty_buffer(id: BufferId) -> Buffer {
        let now = now_utc();

        Buffer {
            id: id,
            filename: None,
            title: String::default(),
            data: Rope::from(""),
            is_changed: false,
            created_time_utc: now,
            last_accessed_time_utc: now,
            last_changed_time_utc: now
        }
    }

    pub fn new_empty_buffer(&mut self) -> Buffer {
        self.next_buffer_id += 1;
        Self::empty_buffer(self.next_buffer_id)
    }

    /// Creates a buffer from a filename. If there is already a Buffer for the file it is returned,
    /// else the file is opened and loaded if it exists, else if the file does not exist then a
    /// new buffer is created with that filename, but no loading is done (the Buffer is considered
    /// to be backed by a file that does not exist yet, it will be created when you save it.)
    pub fn open_file<P: AsRef<Path>>(&mut self, filename: P) -> Buffer {
        let filename = PathBuf::from(filename.as_ref());
        let title = filename.to_string_lossy().into_owned();
        let now = now_utc();
        self.next_buffer_id += 1;
        let contents = fs::load_to_string(&filename).unwrap_or_default();

        Buffer {
            id: self.next_buffer_id,
            filename: Some(filename),
            title: title,
            data: Rope::from(contents),
            is_changed: false,
            created_time_utc: now,
            last_accessed_time_utc: now,
            last_changed_time_utc: now
        }
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;

    #[test]
    fn new_empty_buffer_increments_buffer_id() {
        let mut fac = BufferFactory::new();
        let b1 = fac.new_empty_buffer();
        let b2 = fac.new_empty_buffer();
        assert!(b2.id == b1.id + 1);
    }
}
