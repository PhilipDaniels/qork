use std::path::Path;
use std::slice::Iter;
use std::ops::{Index, IndexMut};

use super::Buffer;

/// Creates, manages and deletes all the buffers in Qork, maintaining the various invariants that
/// we expect from the buffers. Firstly, if a buffer is backed by a file, a second buffer on that
/// file cannot be created. Secondly, all buffers have unique identity.
///
/// Note that a Buffer is very different from a BufferView.
pub struct BufferCollection {
    buffers: Vec<Buffer>
}

impl BufferCollection {
    pub fn new() -> BufferCollection {
        BufferCollection {
            buffers: Vec::new()
        }
    }

    /*
    /// Creates a buffer from a filename. If there is already a Buffer for the file it is returned,
    /// else the file is opened and loaded if it exists, else if the file does not exist then a
    /// new buffer is created with that filename, but no loading is done (the Buffer is considered
    /// to be backed by a file that does not exist yet, it will be created when you save it.)
    pub fn open_file(&mut self) -> Buffer {
        Buffer::new()
    }
    */

    pub fn len(&mut self) -> usize {
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

    pub fn find_by_filename<P : AsRef<Path>>(&mut self, filename: P) -> Option<&mut Buffer> {
        self.buffers.iter_mut().find(|b| b.filename.as_ref().map_or(false, |f| f == filename.as_ref()))
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
mod buffer_collection_tests {
    use super::*;
    use std::path::PathBuf;

    /*
    #[test]
    fn add_works() {
        let mut bc = BufferCollection::new();
        let mut b = Buffer::new();
        b.filename = Some(PathBuf::from("a"));
        bc.add(b);

        let b2 = &bc[0];
        assert_eq!(b2.filename, Some(PathBuf::from("a")));
    }
    */

    #[test]
    fn new_empty_buffer_adds_and_returns_buffer() {
        let now = now_utc();
        let mut bc = BufferCollection::new();

        // let b1 = Buffer::new();
        // let b2 = Buffer::new();
        // bc.add(b1);
        // bc.add(b2);

        let b1 = bc.new_empty_buffer();
        b2 = bc.new_empty_buffer();
        // let i = bc.len();
        // assert_eq!(0, b1.id);
        // assert_eq!(1, i);

        // let b1 = Buffer {
        //     id: 2,
        //     filename: None,
        //     title: String::default(),
        //     data: Rope::from(""),
        //     is_changed: false,
        //     created_time_utc: now,
        //     last_accessed_time_utc: now,
        //     last_changed_time_utc: now
        // };

        // let b2 = Buffer {
        //     id: 2,
        //     filename: None,
        //     title: String::default(),
        //     data: Rope::from(""),
        //     is_changed: false,
        //     created_time_utc: now,
        //     last_accessed_time_utc: now,
        //     last_changed_time_utc: now
        // };


        //let b2 = &bc[0];
        //assert_eq!(b1, b2);
    }
}
