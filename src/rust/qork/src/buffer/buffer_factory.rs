use fs;
use std::path::{Path, PathBuf};
use time::now_utc;
use xi_rope::Rope;
use super::Buffer;

pub struct BufferFactory {
    next_buffer_id: u64
}

impl BufferFactory {
    pub fn new() -> BufferFactory {
        BufferFactory { next_buffer_id: 0 }
    }

    pub fn new_empty_buffer(&mut self) -> Buffer {
        self.next_buffer_id += 1;

        let now = now_utc();

        Buffer {
            id: self.next_buffer_id,
            filename: None,
            title: String::default(),
            data: Rope::from(""),
            is_changed: false,
            created_time_utc: now,
            last_accessed_time_utc: now,
            last_changed_time_utc: now
        }
    }

    pub fn open_file<P: AsRef<Path>>(&mut self, filename: P) -> Option<Buffer> {
        let filename = filename.as_ref();
        let now = now_utc();

        match fs::load_to_string(filename) {
            Ok(contents) => {
                self.next_buffer_id += 1;

                Some(Buffer {
                    id: self.next_buffer_id,
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
}
