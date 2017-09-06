use fs;
use std::path::{Path, PathBuf};
use std::slice::Iter;
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
    // created_time_utc: Tm
    // last_accessed_time_utc: Tm
    // is_changed: bool
    // default_title: String   title to be used when there is only 1 view on a buffer. The filename or Untitled<1>
    // TODO: Need title allocation function
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
}


/// Manages the collection of open buffers.
/// If a buffer is backed by a file, a second buffer on that file cannot be created.
pub struct BufferCollection {
    buffers: Vec<Buffer>
}

impl BufferCollection {
    pub fn new() -> BufferCollection {
        BufferCollection {
            buffers: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    pub fn iter(&self) -> Iter<Buffer> {
        self.buffers.iter()
    }

    pub fn add(&mut self, buffer: Buffer) {
        self.buffers.push(buffer)
    }

    pub fn find_by_filename(&mut self, filename: &str) -> &mut Buffer {
        &mut self.buffers[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::{SeekFrom};

}
