use fs;
use std::path::{Path, PathBuf};
use std::slice::Iter;
use std::ops::{Index, IndexMut};
use std::iter::Iterator;
use xi_rope::Rope;

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

    pub fn open_file(filename: &Path) -> Option<Buffer> {
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

    pub fn find_by_filename(&mut self, filename: &PathBuf) -> Option<&mut Buffer> {
        let pb = Some(filename.clone());

        for b in self.buffers.iter_mut() {
            if b.filename == pb {
                return Some(b);
            }
        }

        //let x = self.buffers.iter().filter_map(|b| b.filename == Some(pb) );

            //.filter(|b| b.filename == Some(pb)).take(1).nth(0);

        //let x = self.buffers.iter()
        //    .filter(|b| b.filename == Some(pb)).take(1).nth(0);

        None

        // for b in self.buffers.iter_mut() {
        //     if b.filename.is_some() {

        //     }
        //     // if let Some(fn) = b.filename {

        //     // }
        //     // .unwrap_or_default() == _filename.as_ref() {
        //     //     return Some(b);
        //     // }
        // }

        // None
    }
}

impl Index<usize> for BufferCollection {
    type Output = Buffer;

    fn index(&self, index: usize) -> &Buffer {
        &self.buffers[index]
    }
}

impl IndexMut<usize> for BufferCollection {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Buffer {
        &mut self.buffers[index]
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;
}

#[cfg(test)]
mod buffer_collection_tests {
    use super::*;

    #[test]
    fn add_works() {
        let mut bc = BufferCollection::new();
        let mut b = Buffer::new();
        b.filename = Some(PathBuf::from("a"));
        bc.add(b);

        let b2 = &bc[0];
        assert_eq!(b2.filename, Some(PathBuf::from("a")));
    }
}
