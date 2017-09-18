use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::cell::{RefCell, RefMut};
use std::ops::{Index, IndexMut};
use std::path::Path;

use super::Buffer;

/// Creates, manages and deletes all the buffers in Qork, maintaining the various invariants that
/// we expect from the buffers. Firstly, if a buffer is backed by a file, a second buffer on that
/// file cannot be created. Secondly, all buffers have unique identity.
///
/// Note that a Buffer is very different from a BufferView.
pub struct BufferCollection {
    buffers: HashMap<u64, RefCell<Buffer>>
}

impl BufferCollection {
    pub fn new() -> BufferCollection {
        BufferCollection {
            buffers: HashMap::with_capacity(20)
        }
    }

    pub fn len(&mut self) -> usize {
        self.buffers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    pub fn get(&self, buffer_id: u64) -> Option<&RefCell<Buffer>> {
         self.buffers.get(&buffer_id)
    }

    pub fn iter<'a>(&'a self) -> Keys<u64, RefCell<Buffer>> {
         let x = self.buffers.keys();
         x
    }

    pub fn insert(&mut self, buffer: Buffer) {
        self.buffers.insert(buffer.id(), RefCell::new(buffer));
    }

    pub fn find_by_filename<P : AsRef<Path>>(&mut self, filename: P) -> Option<&RefCell<Buffer>> {
        self.buffers.values().find(|refcell| refcell.borrow().filename.as_ref().map_or(false, |f| f == filename.as_ref()))
    }
}

impl Index<u64> for BufferCollection {
    type Output = RefCell<Buffer>;

    fn index(&self, buffer_id: u64) -> &RefCell<Buffer> {
        &self.buffers[&buffer_id]
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

    /*
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
    */
}
