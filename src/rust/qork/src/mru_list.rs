use std::cmp;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write, Seek, SeekFrom};
use std::io::prelude::*;
use std::ops::{Index};
use std::path::Path;
use std::slice::{Iter};
use tempfile::{tempfile, NamedTempFile};

use file;

/// A simple MRU-list data structure. Create a list of the appropriate
/// maximum size (which can be changed later) then use `insert` to add new
/// items. New items are always added at the front of the list. Adding
/// an item which is already in the list is ok - it is moved to the beginning
/// of the list. The list keeps track of whether its contents have changed,
/// to allow users to only persist the list if it actually changes.
///
/// The MRUList is not intended to be a high-performance data
/// structure, it is intended for managing small numbers of items such as
/// might appear in an editor's MRU menu.
pub struct MRUList {
    is_changed: bool,
    max_items: usize,
    data: Vec<String>
}


impl MRUList {
    pub fn new(max_items: usize) -> MRUList {
        MRUList {
            is_changed: false,
            max_items: max_items,
            data: Vec::with_capacity(max_items)
        }
    }

    pub fn new_from_slice<S : AsRef<str>>(max_items: usize, src: &[S]) -> MRUList {
        let mut mru = MRUList::new(max_items);

        if !src.is_empty() {
            let max_i_exclusive = cmp::min(max_items, src.len());

            for i in (0..max_i_exclusive).rev() {
                let s2 = src[i].as_ref().to_owned();
                mru.data.insert(0, s2);
            }
        }

        mru
    }

    pub fn is_changed(&self) -> bool {
        self.is_changed
    }

    pub fn clear_is_changed(&mut self) {
        self.is_changed = false;
    }

    pub fn insert<S : Into<String>>(&mut self, value: S) {
        let value = value.into();
        self.remove(&value);
        self.data.insert(0, value);
        self.data.truncate(self.max_items);
        self.is_changed = true;
    }

    pub fn remove(&mut self, value: &str) {
        let pos = self.data.iter().position(|v| v == value);
        if let Some(idx) = pos {
            self.data.remove(idx);
            self.is_changed = true;
        }
    }

    pub fn max_items(&self) -> usize {
        self.max_items
    }

    pub fn set_max_items(&mut self, max_items: usize) {
        if max_items < self.data.len() {
            self.data.truncate(max_items);
            self.is_changed = true;
        }

        self.max_items = max_items;
    }

    pub fn clear(&mut self) {
        if !self.data.is_empty() {
            self.data.clear();
            self.is_changed = true;
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> Iter<String> {
        self.data.iter()
    }

    pub fn write<T:Write>(&mut self, dest: &mut T) -> Result<usize, String> {
        let mut f = BufWriter::new(dest);
        let mut byte_count = 0;

        for v in self.data.iter() {
            let s = format!("{}\n", v);
            byte_count += s.len();
            match write!(&mut f, "{}", s) {
                Ok(_) => {},
                Err(e) => return Err(e.to_string())
            };
        }

        f.flush();

        Ok(byte_count)
    }

    pub fn save(&mut self, filename: &Path) -> Result<usize, String> {
        if self.is_changed {
            return File::create(filename)
                .map_err(|err| err.to_string())
                .and_then(|mut f| {
                    let result = self.write(&mut f);
                    self.is_changed = false;
                    result
                    }
                );
        }

        Ok(0)
    }

    pub fn read<T:Read>(max_mru_items: usize, src: &mut T) -> Result<MRUList, String> {
        let mut f = BufReader::new(src);
        let data = f.lines().take(max_mru_items + 1).map(|l| l.unwrap()).collect::<Vec<String>>();
        let mut mru = MRUList::new_from_slice(max_mru_items, &data);
        mru.is_changed = data.len() > max_mru_items;
        Ok(mru)
    }

    pub fn load(max_mru_items: usize, filename: &Path) -> Result<MRUList, String> {
        File::open(filename)
            .map_err(|err| err.to_string())
            .and_then(|mut f| { MRUList::read(max_mru_items, &mut f) })
    }
}

impl Index<usize> for MRUList {
    type Output = String;

    fn index(&self, index: usize) -> &String {
        &self.data[index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_MRU_AS_STRING : &'static str = "c\nb\na\n";

    fn make_simple_mru() -> MRUList {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.insert("b");
        mru.insert("c");
        mru
    }

    #[test]
    fn write_for_empty_list_writes_no_data() {
        let mut v = Vec::<u8>::new();
        let mut mru = MRUList::new(0);
        mru.write(&mut v);

        assert!(v.is_empty());
    }

    #[test]
    fn write_for_non_empty_list_writes_data() {
        let mut v = Vec::<u8>::new();
        let mut mru = make_simple_mru();
        let cnt = mru.write(&mut v).unwrap();

        let output = String::from_utf8(v).unwrap();
        assert_eq!(output, SIMPLE_MRU_AS_STRING);
        assert_eq!(output.len(), cnt);
    }

    #[test]
    fn save_if_mru_is_changed_writes_file() {
        let mut mru = make_simple_mru();
        let mut file = NamedTempFile::new().expect("failed to create temporary file");
        let cnt = mru.save(file.path()).unwrap();

        assert!(!mru.is_changed());

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut output = String::new();
        file.read_to_string(&mut output);
        assert_eq!(output, SIMPLE_MRU_AS_STRING);
        assert_eq!(output.len(), cnt);
    }

    #[test]
    fn save_if_mru_is_not_changed_does_not_write_file() {
        let mut mru = make_simple_mru();
        let mut file = NamedTempFile::new().expect("failed to create temporary file");
        mru.clear_is_changed();
        let cnt = mru.save(file.path()).unwrap();
        assert_eq!(cnt, 0);

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut output = String::new();
        file.read_to_string(&mut output);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn load_for_file_that_does_not_exist_returns_error() {
        let result = MRUList::load(0, Path::new("/i/definitely/do/not/exist"));
        assert!(result.is_err());
    }

    #[test]
    fn load_for_valid_file_and_max_more_than_file_lines_loads_whole_list_in_correct_order() {
        let mut mru = make_simple_mru();

        let mut file = NamedTempFile::new().expect("failed to create temporary file");
        let cnt = mru.save(file.path()).unwrap();
        assert_eq!(SIMPLE_MRU_AS_STRING.len(), cnt, "This assert checks the file was written correctly");

        let mru = MRUList::load(20, file.path()).unwrap();
        assert_eq!(mru[0], "c", "c is pushed last, so should be the first item in the list");
        assert_eq!(mru[1], "b");
        assert_eq!(mru[2], "a");
        assert_eq!(mru.len(), 3);
        assert!(!mru.is_changed(), "We loaded the entire file, so the MRU does not need truncating, so is_changed should be false");
    }

    #[test]
    fn load_for_valid_file_and_max_less_than_file_lines_loads_only_needed_lines_in_correct_order_and_mru_is_changed() {
        let mut mru = make_simple_mru();

        let mut file = NamedTempFile::new().expect("failed to create temporary file");
        let cnt = mru.save(file.path()).unwrap();
        assert_eq!(SIMPLE_MRU_AS_STRING.len(), cnt, "This assert checks the file was written correctly");

        let mru = MRUList::load(2, file.path()).unwrap();
        assert_eq!(mru[0], "c", "c is pushed last, so should be the first item in the list");
        assert_eq!(mru[1], "b");
        assert_eq!(mru.len(), 2);
        assert!(mru.is_changed(), "We loaded only the beginning of the file, so the MRU needs truncating, so is_changed should be true");
    }

    #[test]
    fn new_for_zero_size_creates_empty_list() {
        let mut mru = MRUList::new(0);
        assert_eq!(mru.len(), 0);
        assert!(mru.is_empty());
        assert!(!mru.is_changed());

        mru.insert("a");
        assert_eq!(mru.len(), 0, "Since max_items is zero, pushing a new element should not increase the length");
        assert!(mru.is_empty());
    }

    #[test]
    fn new_for_size_of_one_creates_list() {
        let mut mru = MRUList::new(1);
        assert_eq!(mru.len(), 0);
        assert!(mru.is_empty());
        assert!(!mru.is_changed());

        mru.insert("a");
        assert_eq!(mru.len(), 1);
        assert!(!mru.is_empty());

        mru.insert("b");
        assert_eq!(mru.len(), 1, "Since max_items is 1, pushing a 2nd element should not increase the length");
    }

    #[test]
    fn new_from_slice_for_empty_slice_creates_list() {
        let src = Vec::<String>::new();
        let mut mru = MRUList::new_from_slice(20, &src);

        assert_eq!(mru.max_items, 20);
        assert_eq!(mru.len(), 0);
        assert!(!mru.is_changed());
    }

    #[test]
    fn new_from_slice_for_slice_with_one_item_creates_list_with_one_item() {
        let src = ["a"];
        let mut mru = MRUList::new_from_slice(20, &src);

        assert_eq!(mru.max_items, 20);
        assert_eq!(mru.len(), 1);
        assert!(!mru.is_changed());
        assert_eq!(mru[0], "a");
    }

    #[test]
    fn new_from_slice_for_non_empty_slice_creates_list_with_items_in_same_order() {
        let src = ["a", "b", "c"];
        let mut mru = MRUList::new_from_slice(20, &src);

        assert_eq!(mru.max_items, 20);
        assert_eq!(mru.len(), 3);
        assert!(!mru.is_changed());
        assert_eq!(mru[0], "a");
        assert_eq!(mru[1], "b");
        assert_eq!(mru[2], "c");
    }

    #[test]
    fn new_from_slice_for_zero_max_items_and_empty_slice_takes_no_items() {
        let src = Vec::<String>::new();
        let mut mru = MRUList::new_from_slice(0, &src);

        assert_eq!(mru.max_items, 0);
        assert_eq!(mru.len(), 0);
        assert!(!mru.is_changed());
    }

    #[test]
    fn new_from_slice_for_zero_max_items_takes_no_items() {
        let src = ["a", "b", "c"];
        let mut mru = MRUList::new_from_slice(0, &src);

        assert_eq!(mru.max_items, 0);
        assert_eq!(mru.len(), 0);
        assert!(!mru.is_changed());
    }

    #[test]
    fn new_from_slice_for_max_items_less_than_slice_length_takes_only_requested_items() {
        let src = ["a", "b", "c"];
        let mut mru = MRUList::new_from_slice(2, &src);

        assert_eq!(mru.max_items, 2);
        assert_eq!(mru.len(), 2);
        assert!(!mru.is_changed());
        assert_eq!(mru[0], "a");
        assert_eq!(mru[1], "b");
    }

    #[test]
    fn new_from_slice_works_for_owned_strings() {
        // I wrote this test when I changed new_from_slice to use AsRef, since
        // it was a new technique to me. It duplicates another test, and hence
        // mainly checks that things compile.
        let src = ["a".to_owned(), "b".to_owned(), "c".to_owned()];
        let mut mru = MRUList::new_from_slice(2, &src);

        assert_eq!(mru.max_items, 2);
        assert_eq!(mru.len(), 2);
        assert!(!mru.is_changed());
        assert_eq!(mru[0], "a");
        assert_eq!(mru[1], "b");
    }

    #[test]
    fn is_empty_for_empty_list_returns_true() {
        let mut mru = MRUList::new(0);
        assert!(mru.is_empty());

        let mut mru = MRUList::new(1);
        assert!(mru.is_empty());
    }

    #[test]
    fn push_sets_is_changed_flag() {
        let mut mru = MRUList::new(20);
        mru.insert("a");

        assert!(mru.is_changed());
    }

    #[test]
    fn push_adds_items_in_push_down_stack_order() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.insert("b");

        assert_eq!(mru[0], "b", "b was pushed last, so should be at the head of the list");
        assert_eq!(mru[1], "a", "a was pushed before b, so should be the second item");
    }

    #[test]
    fn push_for_item_already_in_list_moves_item_to_front() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.insert("b");
        mru.insert("c");
        mru.insert("a");

        assert_eq!(mru[0], "a");
        assert_eq!(mru[1], "c");
        assert_eq!(mru[2], "b");
        assert_eq!(mru.len(), 3);
    }

    #[test]
    fn push_for_list_at_capacity_drops_items_off_end() {
        let mut mru = MRUList::new(2);
        mru.insert("a");
        mru.insert("b");
        mru.insert("c");

        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
        assert_eq!(mru.len(), 2);
    }

    #[test]
    fn remove_for_item_not_in_list_does_nothing() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.insert("b");
        mru.remove("c");
        mru.clear_is_changed();

        assert!(!mru.is_changed());
        assert_eq!(mru[0], "b");
        assert_eq!(mru[1], "a");
        assert_eq!(mru.len(), 2);
    }

    #[test]
    fn remove_for_list_of_one_item_removes_item() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.remove("a");

        assert!(mru.is_changed());
        assert!(mru.is_empty());
    }

    #[test]
    fn remove_for_list_of_several_items_with_item_at_end_removes_item() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.insert("b");
        mru.insert("c");
        mru.remove("a");

        assert!(mru.is_changed());
        assert_eq!(mru.len(), 2);
        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
    }

    #[test]
    fn remove_for_list_of_several_items_with_item_at_beginning_removes_item() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.insert("b");
        mru.insert("c");
        mru.remove("c");

        assert!(mru.is_changed());
        assert_eq!(mru.len(), 2);
        assert_eq!(mru[0], "b");
        assert_eq!(mru[1], "a");
    }

    #[test]
    fn remove_for_list_of_several_items_with_item_in_middle_removes_item() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.insert("b");
        mru.insert("c");
        mru.remove("b");

        assert!(mru.is_changed());
        assert_eq!(mru.len(), 2);
        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "a");
    }

    #[test]
    fn set_max_items_sets_is_changed_flag() {
        let mut mru = MRUList::new(20);
        assert!(!mru.is_changed());

        mru.set_max_items(3);
        assert!(!mru.is_changed(), "is_changed should still be false, because we are increasing the size of the list, which is a no-op");

        mru.set_max_items(2);
        assert!(!mru.is_changed(), "is_changed should still be false, because we are decreasing the size of the list, but it is currently empty, so this is a no-op");

        mru.insert("a");
        mru.insert("b");
        mru.clear_is_changed();

        mru.set_max_items(1);
        assert!(mru.is_changed(), "is_changed should be true, because we shrank the number of elements in the list");
    }

    #[test]
    fn set_max_items_for_new_size_smaller_than_current_trims_list_to_size() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.insert("b");
        mru.insert("c");

        mru.set_max_items(2);

        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
        assert_eq!(mru.len(), 2);
    }

    #[test]
    fn set_max_items_for_new_size_greater_than_current_leaves_list_untouched() {
        let mut mru = MRUList::new(3);
        mru.insert("a");
        mru.insert("b");
        mru.insert("c");

        mru.set_max_items(20);

        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
        assert_eq!(mru[2], "a");
        assert_eq!(mru.len(), 3);
    }

    #[test]
    fn clear_for_empty_list_does_not_panic_and_does_not_set_the_changed_flag() {
        let mut mru = MRUList::new(20);
        mru.clear();

        assert!(mru.is_empty());
        assert!(!mru.is_changed(), "We should not mark an empty list as changed.");
    }

    #[test]
    fn clear_for_non_empty_list_clears_list() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.clear();

        assert!(mru.is_changed());
        assert!(mru.is_empty());
    }

    #[test]
    fn iter_for_empty_list_returns_zero_items() {
        let mut mru = MRUList::new(0);
        let mut iter = mru.iter();
        assert_eq!(iter.next(), None);

        let mut mru = MRUList::new(1);
        let mut iter = mru.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_for_list_with_items_returns_items_in_correct_order() {
        let mut mru = MRUList::new(20);
        mru.insert("a");
        mru.insert("b");
        mru.insert("c");

        let mut iter = mru.iter();
        assert_eq!(iter.next(), Some(&"c".to_owned()));
        assert_eq!(iter.next(), Some(&"b".to_owned()));
        assert_eq!(iter.next(), Some(&"a".to_owned()));
        assert_eq!(iter.next(), None);
    }
}
