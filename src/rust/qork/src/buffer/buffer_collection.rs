use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::cell::RefCell;
use std::ops::Index;
use std::path::Path;

use super::{Buffer, BufferId};

/// Creates, manages and deletes all the buffers in Qork, maintaining the various invariants that
/// we expect from the buffers. Firstly, if a buffer is backed by a file, a second buffer on that
/// file cannot be created. Secondly, all buffers have unique identity.
///
/// Note that a Buffer is very different from a BufferView.
pub struct BufferCollection {
    current_buffer: BufferId,
    buffers: HashMap<BufferId, RefCell<Buffer>>
}

impl BufferCollection {
    pub fn new() -> BufferCollection {
        BufferCollection {
            current_buffer: -1,
            buffers: HashMap::with_capacity(20)
        }
    }

    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    pub fn current_buffer(&self) -> BufferId {
        self.current_buffer
    }

    pub fn set_current_buffer(&mut self, buffer_id: BufferId) -> bool {
        if self.buffers.contains_key(&buffer_id) {
            self.current_buffer = buffer_id;
            return true;
        } else {
            return false;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    pub fn get(&self, buffer_id: BufferId) -> Option<&RefCell<Buffer>> {
         self.buffers.get(&buffer_id)
    }

    pub fn keys(&self) -> Keys<BufferId, RefCell<Buffer>> {
         self.buffers.keys()
    }

    pub fn insert(&mut self, buffer: Buffer) {
        self.buffers.insert(buffer.id(), RefCell::new(buffer));
    }

    pub fn remove(&mut self, buffer_id: BufferId) -> Option<RefCell<Buffer>> {
        self.buffers.remove(&buffer_id)
    }

    pub fn find_by_filename<P : AsRef<Path>>(&mut self, filename: P) -> Option<&RefCell<Buffer>> {
        self.buffers.values().find(|refcell| refcell.borrow().filename.as_ref().map_or(false, |f| f == filename.as_ref()))
    }

    // Needs a buffer factory passed in. Check to see if the file is already open. If it is, return
    // the id of the first buffer rather than creating a new one.
    pub fn open_file() {}
    // Saves a buffer, but only if it already has a filename. Set changed to false.
    pub fn save_buffer() {}
    // Saves a buffer to a specific filename. Might need to allocate a new title. Can change
    // the filename that a buffer is saved to. Set changed to false.
    pub fn save_buffer_as() {}
    // Not sure we need this, it is really just remove disguised.
    pub fn close_buffer() {}

    // We also need a filename pre-processor better than just expand_variables.
    // It should be able to deal with relative names, trying to find a file relative to the
    // current buffer's directory (for working in projects), or failing that, to the cwd.


    /// Title algorithm. We need the ability to uniqueify buffer names. Once a suffix number is
    /// assigned it is never changed. They can be reused, or even not used (we only allocate them
    /// for our convenience, and they do not need to go up monotonically like buffer ids do).
    /// For new buffers not backed by a file "new", then "new 1 " etc.
    /// For buffers backed by a file, the leaf filename, then '1' etc.
    fn get_unique_title(&self, proposed: &str) -> String {
        // let prefix = String::from(proposed) + " ";
        // let matching_buffers : Vec<String> = self.buffers.values()
        //     .map(|refcell| refcell.borrow())
        //     .map(|x| x.title.clone())
        //     .filter(|title| *title == String::from(proposed) || title.starts_with(&prefix))
        //     .collect();

        let titles : Vec<_> = self.buffers.values().map(|refcell| refcell.borrow().title.clone()).collect();

        //let titles2 : Vec<_> = self.buffers.values().map(|refcell| *(refcell.borrow().title)).collect();

        inner_get_unique_title(proposed, &titles)
    }
}

fn inner_get_unique_title(proposed: &str, current_titles: &[String]) -> String {
    let namer = |n: i32| -> String {
        if n == 0 {
            return String::from(proposed)
        } else {
            return String::from(proposed) + " " + &n.to_string()
        }
    };

    let mut i = 0;
    loop {
        let p = namer(i);
        if !current_titles.contains(&p) {
            return p
        } else {
            i += 1
        }
    }
}

impl Index<BufferId> for BufferCollection {
    type Output = RefCell<Buffer>;

    fn index(&self, buffer_id: BufferId) -> &RefCell<Buffer> {
        &self.buffers[&buffer_id]
    }
}

#[cfg(test)]
mod buffer_collection_tests {
    use super::*;
    use super::super::BufferFactory;

    #[test]
    fn get_unique_title_for_empty_collection_returns_proposed() {
        let bc = BufferCollection::new();
        let proposed = "new";
        let title = bc.get_unique_title(proposed);
        assert_eq!(title, proposed);
    }

    #[test]
    fn get_unique_title_for_collection_with_no_matching_buffer_title_returns_proposed() {
        let mut bc = BufferCollection::new();
        let mut fac = BufferFactory::new();

        let mut b = fac.new_empty_buffer();
        b.title = String::from("t1");
        bc.insert(b);

        let mut b = fac.new_empty_buffer();
        b.title = String::from("t2");
        bc.insert(b);

        let proposed = "new";
        let title = bc.get_unique_title(proposed);
        assert_eq!(title, proposed);
    }

    #[test]
    fn get_unique_title_for_collection_with_matching_buffer_title_returns_proposed_with_numeral_of_one() {
        let mut bc = BufferCollection::new();
        let mut fac = BufferFactory::new();

        let mut b = fac.new_empty_buffer();
        b.title = String::from("new");
        bc.insert(b);

        let proposed = "new";
        let title = bc.get_unique_title(proposed);
        assert_eq!(title, "new 1");
    }

    #[test]
    fn get_unique_title_for_collection_with_matching_buffer_titles_in_ascending_order_returns_proposed_with_next_free_numeral() {
        let mut bc = BufferCollection::new();
        let mut fac = BufferFactory::new();

        let mut b = fac.new_empty_buffer();
        b.title = String::from("new");
        bc.insert(b);

        let mut b = fac.new_empty_buffer();
        b.title = String::from("new 1");
        bc.insert(b);

        let mut b = fac.new_empty_buffer();
        b.title = String::from("new 2");
        bc.insert(b);

        let proposed = "new";
        let title = bc.get_unique_title(proposed);
        assert_eq!(title, "new 3");
    }

    #[test]
    fn get_unique_title_for_collection_with_matching_buffer_titles_with_gap_returns_proposed_with_lowest_free_numeral() {
        let mut bc = BufferCollection::new();
        let mut fac = BufferFactory::new();

        let mut b = fac.new_empty_buffer();
        b.title = String::from("new");
        bc.insert(b);

        let mut b = fac.new_empty_buffer();
        b.title = String::from("new 1");
        bc.insert(b);

        let mut b = fac.new_empty_buffer();
        b.title = String::from("new 3");
        bc.insert(b);

        let proposed = "new";
        let title = bc.get_unique_title(proposed);
        assert_eq!(title, "new 2");
    }

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
