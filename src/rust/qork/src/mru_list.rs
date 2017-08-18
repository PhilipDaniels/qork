use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

/// A simple MRU-list data structure. Create a list of the appropriate
/// maximum size (which can be changed later) then use `push` to add new
/// items. New items are always added at the front of the list. Adding
/// an item which is already in the list is ok - it is moved to the beginning
/// of the list. The list keeps track of whether its contents have changed,
/// to allow users to only persist the list if it actually changes.
///
/// The MRUList is not intended to be a high-performance data
/// structure, it is intended for managing small numbers of items such as
/// might appear in an editor's MRU menu.
pub struct MRUList<T> {
    is_changed: bool,
    max_items: usize,
    data: Vec<T>
}

impl<T> MRUList<T>
    // This constaint is required by the `remove` method.
    where T: Eq {

    pub fn new(max_items: usize) -> MRUList<T> {
        MRUList {
            is_changed: false,
            max_items: max_items,
            data: Vec::<T>::with_capacity(max_items)
        }
    }

    pub fn is_changed(&self) -> bool {
        self.is_changed
    }

    pub fn clear_is_changed(&mut self) {
        self.is_changed = false;
    }

    pub fn push(&mut self, value: T) {
        self.remove(&value);
        self.data.insert(0, value);
        self.data.truncate(self.max_items);
        self.is_changed = true;
    }

    pub fn remove(&mut self, value: &T) {
        let pos = self.data.iter().position(|v| v == value);
        if let Some(idx) = pos {
            self.data.remove(idx);
            self.is_changed = true;
        }
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

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
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



// Run the tests using String since that is what we are likely to be using this class for.
// This makes them a little more verbose than using int or str but is worth it.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_for_zero_size_creates_empty_list() {
        let mut mru = MRUList::new(0);
        assert_eq!(mru.len(), 0);
        assert!(mru.is_empty());
        assert!(!mru.is_changed());

        mru.push("a".to_owned());
        assert_eq!(mru.len(), 0, "Since max_items is zero, pushing a new element should not increase the length");
        assert!(mru.is_empty());
    }

    #[test]
    fn new_for_size_of_one_creates_list() {
        let mut mru = MRUList::new(1);
        assert_eq!(mru.len(), 0);
        assert!(mru.is_empty());
        assert!(!mru.is_changed());

        mru.push("a".to_owned());
        assert_eq!(mru.len(), 1);
        assert!(!mru.is_empty());

        mru.push("b".to_owned());
        assert_eq!(mru.len(), 1, "Since max_items is 1, pushing a 2nd element should not increase the length");
    }

    #[test]
    fn is_empty_for_empty_list_returns_true() {
        let mut mru = MRUList::<String>::new(0);
        assert!(mru.is_empty());

        let mut mru = MRUList::<String>::new(1);
        assert!(mru.is_empty());
    }

    #[test]
    fn push_sets_is_changed_flag() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());

        assert!(mru.is_changed());
    }

    #[test]
    fn push_adds_items_in_push_down_stack_order() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.push("b".to_owned());

        assert_eq!(mru[0], "b", "b was pushed last, so should be at the head of the list");
        assert_eq!(mru[1], "a", "a was pushed before b, so should be the second item");
    }

    #[test]
    fn push_for_item_already_in_list_moves_item_to_front() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.push("c".to_owned());
        mru.push("a".to_owned());

        assert_eq!(mru[0], "a");
        assert_eq!(mru[1], "c");
        assert_eq!(mru[2], "b");
        assert_eq!(mru.len(), 3);
    }

    #[test]
    fn push_for_list_at_capacity_drops_items_off_end() {
        let mut mru = MRUList::new(2);
        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.push("c".to_owned());

        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
        assert_eq!(mru.len(), 2);
    }

    #[test]
    fn remove_for_item_not_in_list_does_nothing() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.remove(&"c".to_owned());
        mru.clear_is_changed();

        assert!(!mru.is_changed());
        assert_eq!(mru[0], "b");
        assert_eq!(mru[1], "a");
        assert_eq!(mru.len(), 2);
    }

    #[test]
    fn remove_for_list_of_one_item_removes_item() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.remove(&"a".to_owned());

        assert!(mru.is_changed());
        assert!(mru.is_empty());
    }

    #[test]
    fn remove_for_list_of_several_items_with_item_at_end_removes_item() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.push("c".to_owned());
        mru.remove(&"a".to_owned());

        assert!(mru.is_changed());
        assert_eq!(mru.len(), 2);
        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
    }

    #[test]
    fn remove_for_list_of_several_items_with_item_at_beginning_removes_item() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.push("c".to_owned());
        mru.remove(&"c".to_owned());

        assert!(mru.is_changed());
        assert_eq!(mru.len(), 2);
        assert_eq!(mru[0], "b");
        assert_eq!(mru[1], "a");
    }

    #[test]
    fn remove_for_list_of_several_items_with_item_in_middle_removes_item() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.push("c".to_owned());
        mru.remove(&"b".to_owned());

        assert!(mru.is_changed());
        assert_eq!(mru.len(), 2);
        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "a");
    }

    #[test]
    fn set_max_items_sets_is_changed_flag() {
        let mut mru = MRUList::<String>::new(20);
        assert!(!mru.is_changed());

        mru.set_max_items(3);
        assert!(!mru.is_changed(), "is_changed should still be false, because we are increasing the size of the list, which is a no-op");

        mru.set_max_items(2);
        assert!(!mru.is_changed(), "is_changed should still be false, because we are decreasing the size of the list, but it is currently empty, so this is a no-op");

        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.clear_is_changed();

        mru.set_max_items(1);
        assert!(mru.is_changed(), "is_changed should be true, because we shrank the number of elements in the list");
    }

    #[test]
    fn set_max_items_for_new_size_smaller_than_current_trims_list_to_size() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.push("c".to_owned());

        mru.set_max_items(2);

        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
        assert_eq!(mru.len(), 2);
    }

    #[test]
    fn set_max_items_for_new_size_greater_than_current_leaves_list_untouched() {
        let mut mru = MRUList::new(3);
        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.push("c".to_owned());

        mru.set_max_items(20);

        assert_eq!(mru[0], "c");
        assert_eq!(mru[1], "b");
        assert_eq!(mru[2], "a");
        assert_eq!(mru.len(), 3);
    }

    #[test]
    fn clear_for_empty_list_does_not_panic_and_does_not_set_the_changed_flag() {
        let mut mru = MRUList::<String>::new(20);
        mru.clear();

        assert!(mru.is_empty());
        assert!(!mru.is_changed(), "We should not mark an empty list as changed.");
    }

    #[test]
    fn clear_for_non_empty_list_clears_list() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.clear();

        assert!(mru.is_changed());
        assert!(mru.is_empty());
    }

    #[test]
    fn index_mut_changes_item() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru[0] = "b".to_owned();
        assert_eq!(mru.len(), 1);
        assert_eq!(mru[0], "b");
    }

    #[test]
    fn iter_for_empty_list_returns_zero_items() {
        let mut mru = MRUList::<String>::new(0);
        let mut iter = mru.iter();
        assert_eq!(iter.next(), None);

        let mut mru = MRUList::<String>::new(1);
        let mut iter = mru.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_for_list_with_items_returns_items_in_correct_order() {
        let mut mru = MRUList::new(20);
        mru.push("a".to_owned());
        mru.push("b".to_owned());
        mru.push("c".to_owned());

        let mut iter = mru.iter();
        assert_eq!(iter.next(), Some(&"c".to_owned()));
        assert_eq!(iter.next(), Some(&"b".to_owned()));
        assert_eq!(iter.next(), Some(&"a".to_owned()));
        assert_eq!(iter.next(), None);
    }
}
