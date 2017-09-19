use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::cell::RefCell;
use std::ops::Index;
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

    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    pub fn get(&self, buffer_id: u64) -> Option<&RefCell<Buffer>> {
         self.buffers.get(&buffer_id)
    }

    pub fn keys(&self) -> Keys<u64, RefCell<Buffer>> {
         self.buffers.keys()
    }

    pub fn insert(&mut self, buffer: Buffer) {
        self.buffers.insert(buffer.id(), RefCell::new(buffer));
    }

    pub fn remove(&mut self, buffer_id: u64) -> Option<RefCell<Buffer>> {
        self.buffers.remove(&buffer_id)
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
    use super::super::BufferFactory;

    #[test]
    fn is_empty_for_empty_collection_returns_true() {
        let bc = BufferCollection::new();
        assert!(bc.is_empty());
    }

    #[test]
    fn len_for_empty_collection_returns_zero() {
        let bc = BufferCollection::new();
        assert_eq!(0, bc.len());
    }

    #[test]
    fn len_for_non_empty_collection_returns_length() {
        let mut bc = BufferCollection::new();
        let mut fac = BufferFactory::new();
        let b = fac.new_empty_buffer();
        bc.insert(b);

        assert_eq!(1, bc.len());
    }

    #[test]
    fn get_for_empty_collection_returns_false() {
        let bc = BufferCollection::new();
        assert!(bc.get(0).is_none());
    }

    #[test]
    fn get_for_id_not_in_collection_returns_none() {
        let mut bc = BufferCollection::new();
        let mut fac = BufferFactory::new();
        let b = fac.new_empty_buffer();
        let id = b.id;
        bc.insert(b);
        assert!(bc.get(id + 1).is_none());
    }

    #[test]
    fn get_for_id_in_collection_returns_buffer() {
        let mut bc = BufferCollection::new();
        let mut fac = BufferFactory::new();
        let b = fac.new_empty_buffer();
        let id = b.id;
        bc.insert(b);

        let result = bc.get(id).unwrap().borrow();
        assert_eq!(id, result.id());

    }

    #[test]
    fn remove_for_non_existent_buffer_returns_none() {
        let mut bc = BufferCollection::new();
        assert_eq!(bc.remove(1), None);
    }

    #[test]
    fn remove_for_buffer_in_collection_returns_buffer() {
        let mut bc = BufferCollection::new();
        let mut fac = BufferFactory::new();
        let b = fac.new_empty_buffer();
        let id = b.id;
        bc.insert(b);

        let b2 = bc.remove(id).unwrap().into_inner();
        assert_eq!(id, b2.id());
        assert_eq!(0, bc.len());
    }

    #[test]
    fn find_by_filename_for_filename_not_in_collection_returns_none() {
        let mut bc = BufferCollection::new();
        let result = bc.find_by_filename("/c/temp/feeeegfxgdg");
        assert_eq!(result, None);
    }

    #[test]
    fn find_by_filename_for_filename_in_collection_returns_buffer() {
        let mut bc = BufferCollection::new();
        let mut fac = BufferFactory::new();
        let b = fac.open_file("/c/foo.txt");
        let id = b.id;
        bc.insert(b);

        let result = bc.find_by_filename("/c/foo.txt").unwrap();
        assert_eq!(result.borrow().id(), id);
    }
}
