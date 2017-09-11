use std::path::Path;
use std::slice::Iter;
use std::ops::{Index, IndexMut};
use super::Buffer;

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
