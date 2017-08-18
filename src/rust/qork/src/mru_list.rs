use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

/// A simple MRU-list data structure. Create a list of the appropriate
/// maximum size (which can be changed later) then use `push` to add new
/// items. New items are always added at the front of the list. Adding
/// an item which is already in the list is ok - it is moved to the beginning
/// of the list.
pub struct MRUList<T> {
    max_items: usize,
    data: Vec<T>
}

impl<T> MRUList<T>
    where T: Eq {

    pub fn new(max_items: usize) -> MRUList<T> {
        MRUList {
            max_items: max_items,
            data: Vec::<T>::new()
        }
    }

    pub fn push(&mut self, value: T) {
        self.remove(&value);
        self.data.insert(0, value);
        self.data.truncate(self.max_items);
    }

    pub fn remove(&mut self, value: &T) {
        let pos = self.data.iter().position(|v| v == value);
        if let Some(idx) = pos {
            self.data.remove(idx);
        }
    }

    pub fn set_max_items(&mut self, max_items: usize) {
        self.max_items = max_items;
        self.data.truncate(self.max_items);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }
}

impl<T> Index<usize> for MRUList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for MRUList<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        &mut self.data[index]
    }
}

pub fn new_string_mru(max_items: usize) -> MRUList<&'static str> {
    MRUList::<&'static str>::new(max_items)
}


// Run the tests using strings since that is what we are likely to be
// using this class for.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_for_zero_size_creates_empty_list() {
        let mut mru = new_string_mru(0);
        assert_eq!(mru.len(), 0);
        assert!(mru.is_empty());
        mru.push("a");
        assert_eq!(mru.len(), 0, "Since max_items is zero, pushing a new element should not increase the length");
        assert!(mru.is_empty());
    }

    #[test]
    fn new_for_size_of_one_creates_list() {
        let mut mru = new_string_mru(1);
        assert_eq!(mru.len(), 0);
        assert!(mru.is_empty());
        mru.push("a");
        assert_eq!(mru.len(), 1);
        assert!(!mru.is_empty());
        mru.push("b");
        assert_eq!(mru.len(), 1, "Since max_items is 1, pushing a 2nd element should not increase the length");
    }

    #[test]
    fn is_empty_for_empty_list_returns_true() {
        let mut mru = new_string_mru(0);
        assert!(mru.is_empty());
        let mut mru = new_string_mru(1);
        assert!(mru.is_empty());
    }

    #[test]
    fn clear_for_empty_list_does_not_panic() {
        let mut mru = new_string_mru(20);
        mru.clear();
        assert!(mru.is_empty());
    }

    #[test]
    fn clear_for_non_empty_list_clears_list() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru.clear();
        assert!(mru.is_empty());
    }

    #[test]
    fn push_adds_items_in_push_down_stack_order() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru.push("b");
        assert_eq!(mru[0], "b", "b was pushed last, so should be at the head of the list");
        assert_eq!(mru[1], "a", "a was pushed before b, so should be the second item");
    }

    #[test]
    fn push_for_item_already_in_list_moves_item_to_front() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru.push("b");
        mru.push("c");

        mru.push("a");

        assert_eq!(mru[0], "a");
        assert_eq!(mru[1], "c");
        assert_eq!(mru[2], "b");
        assert_eq!(mru.len(), 3);
    }

    #[test]
    fn push_for_list_at_capacity_drops_items_off_end() {
        let mut mru = new_string_mru(2);
        mru.push("a");
        mru.push("b");
        mru.push("c");

        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
        assert_eq!(mru.len(), 2);
    }

    #[test]
    fn remove_for_item_not_in_list_does_nothing() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru.push("b");
        mru.remove(&"c");
        assert_eq!(mru[0], "b");
        assert_eq!(mru[1], "a");
        assert_eq!(mru.len(), 2);
    }

    #[test]
    fn remove_for_list_of_one_item_removes_item() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru.remove(&"a");
        assert!(mru.is_empty());
    }

    #[test]
    fn remove_for_list_of_several_items_with_item_at_end_removes_item() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru.push("b");
        mru.push("c");
        mru.remove(&"a");
        assert_eq!(mru.len(), 2);
        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
    }

    #[test]
    fn remove_for_list_of_several_items_with_item_at_beginning_removes_item() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru.push("b");
        mru.push("c");
        mru.remove(&"c");
        assert_eq!(mru.len(), 2);
        assert_eq!(mru[0], "b");
        assert_eq!(mru[1], "a");
    }

    #[test]
    fn set_max_items_for_new_size_smaller_than_current_trims_list_to_size() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru.push("b");
        mru.push("c");

        mru.set_max_items(2);

        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
        assert_eq!(mru.len(), 2);
    }

    #[test]
    fn set_max_items_for_new_size_greater_than_current_leaves_list_untouched() {
        let mut mru = new_string_mru(3);
        mru.push("a");
        mru.push("b");
        mru.push("c");

        mru.set_max_items(20);

        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
        assert_eq!(mru[2], "a");
        assert_eq!(mru.len(), 3);
    }

    #[test]
    fn index_mut_changes_item() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru[0] = "b";
        assert_eq!(mru.len(), 1);
        assert_eq!(mru[0], "b");
    }

    #[test]
    fn iter_for_empty_list_returns_zero_items() {
        let mut mru = new_string_mru(0);
        let mut iter = mru.iter();
        assert_eq!(iter.next(), None);

        let mut mru = new_string_mru(1);
        let mut iter = mru.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_for_list_with_items_returns_items_in_correct_order() {
        let mut mru = new_string_mru(20);
        mru.push("a");
        mru.push("b");
        mru.push("c");

        let mut iter = mru.iter();
        assert_eq!(iter.next(), Some(&"c"));
        assert_eq!(iter.next(), Some(&"b"));
        assert_eq!(iter.next(), Some(&"a"));
        assert_eq!(iter.next(), None);
    }
}