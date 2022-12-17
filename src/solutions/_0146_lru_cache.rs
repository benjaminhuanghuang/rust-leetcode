/*
    146. LRU Cache

    https://leetcode.com/problems/lru-cache/

    Medium


    * If the number of keys exceeds the capacity from this operation, `evict` the least recently used key.
    * The functions get and put must each run in O(1) average time complexity.
*/

/*
    Least Recently Used, 最近最少使用, 关键在于追踪每一个 entry 的 age, 每次淘汰最旧的那一个 key
    用一个链表记录node被访问的顺序，最新被访问的node放在tail，

    None <--- node <---> node <---> node <---> node <---> node ---> None
             key         key        key        key        key
            value       value      value      value      value
               ^           ^          ^          ^          ^
               |           |          |          |          |
               |           |          |          |          |
     map:     key         key        key        key        key

    get(key): delete entry and push_back to the linked list,
    put(key, value):
        If key exist, delete entry and push_back to the linked list,,
        If not
            If reach capacity, push_back,
            else capacity, pop_front, push_back,
*/
use std::collections::HashMap;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct LRUEntry {
    val: i32,
    key: i32, // when pop_front entry, remove entry.key from map
    next: Option<Rc<RefCell<LRUEntry>>>,
    prev: Option<Weak<RefCell<LRUEntry>>>,
}

impl LRUEntry {
    pub fn new(key: i32, val: i32) -> Self {
        LRUEntry {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

pub struct LRUCache {
    capacity: usize,
    length: usize,
    map: HashMap<i32, Weak<RefCell<LRUEntry>>>,
    head: Option<Rc<RefCell<LRUEntry>>>,
    tail: Option<Rc<RefCell<LRUEntry>>>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let cap = capacity as usize;
        Self {
            map: HashMap::with_capacity(cap),
            length: 0,
            capacity: capacity as usize,
            head: None,
            tail: None,
        }
    }

    /*
        access entry from map and move the entry to back
    */
    pub fn get(&mut self, key: i32) -> i32 {
        let ptr = self.map.get_mut(&key);

        // key does not exist
        if ptr.is_none() {
            return -1;
        }

        // update entry by move_entry_to_back
        let ptr = ptr.unwrap();
        let ptr = ptr.upgrade();
        match ptr {
            None => -1,
            Some(entry_ptr) => {
                let value = entry_ptr.borrow().val;
                self.move_entry_to_back(entry_ptr);
                value
            }
        }
    }

    /*
        if key existed, update the value of the entry and move_entry_to_back
        if not, create a new entry and push new entry to back
    */
    pub fn put(&mut self, key: i32, val: i32) {
        let ptr = self.map.get_mut(&key);
        let ptr = if ptr.is_some() {
            ptr.unwrap().upgrade()
        } else {
            None
        };

        match ptr {
            None => {
                // can be not in the map or null pointer
                // create a new entry and insert it to back
                // check capacity
                let entry = LRUEntry::new(key, val);
                self.push_entry_back(Rc::new(RefCell::new(entry)));
                if let Some(tail) = self.get_weak_tail() {
                    self.map.insert(key, tail);
                }
                if self.length > self.capacity {
                    let key = self.pop_front();
                    self.map.remove(&key.unwrap());
                }
            }
            Some(entry) => {
                // update the value in the entry and move it to back
                entry.borrow_mut().val = val;
                self.move_entry_to_back(entry)
            }
        }
    }

    //-------------------------------
    // Utility functions
    fn get_weak_tail(&self) -> Option<Weak<RefCell<LRUEntry>>> {
        match &self.tail {
            None => None,
            Some(tail) => Some(Rc::downgrade(tail)),
        }
    }

    fn pop_front(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(head) => {
                let mut head = head.borrow_mut();
                let next = head.next.take();

                match next {
                    None => {
                        self.tail.take();
                    }
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next)
                    }
                }
                self.length -= 1;
                Some(head.key)
            }
        }
    }

    fn move_entry_to_back(&mut self, mut entry_ptr: Rc<RefCell<LRUEntry>>) {
        self.remove_entry_from_list(&mut entry_ptr);
        self.push_entry_back(entry_ptr);
    }

    fn push_entry_back(&mut self, mut entry_ptr: Rc<RefCell<LRUEntry>>) {
        match self.tail.take() {
            None => {
                self.head.replace(entry_ptr);
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                entry_ptr
                    .borrow_mut()
                    .prev
                    .replace(Rc::downgrade(&current_tail));

                self.tail.replace(entry_ptr);
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
        self.length += 1;
    }

    fn remove_entry_from_list(&mut self, entry_ptr: &mut Rc<RefCell<LRUEntry>>) {
        // 1. remove the entry from linked-list
        let (prev, next) = {
            let mut node = entry_ptr.borrow_mut();
            let prev = match node.prev.take() {
                None => None,
                Some(prev) => prev.upgrade(),
            };
            let next = node.next.take();
            (prev, next)
        };

        match (prev, next) {
            (None, None) => {
                // entry_ptr is the only entry in the list
                self.head = None;
                self.tail = None;
            }
            (None, Some(next)) => {
                // entry_ptr is head
                next.borrow_mut().prev = None;
                self.head.replace(next);
            }
            (Some(prev), None) => {
                // entry_ptr is tail
                prev.borrow_mut().next = None;
                self.tail.replace(prev);
            }
            (Some(prev), Some(next)) => {
                next.borrow_mut().prev.replace(Rc::downgrade(&prev));
                prev.borrow_mut().next.replace(next);
            }
        }
        self.length -= 1;
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_146() {
        //test1();

        //test2();

        test3();
    }

    fn test1() {
        println!("init cache");
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1); // returns 1

        lru_cache.put(3, 3); // evicts key 2
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        assert_eq!(lru_cache.get(3), 3);

        lru_cache.put(4, 4); // evicts key 1
        assert_eq!(lru_cache.get(1), -1); // returns -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // returns 3
        assert_eq!(lru_cache.get(4), 4); // returns 4
    }

    fn test2() {
        let mut lru_cache = LRUCache::new(2);
        assert_eq!(lru_cache.get(2), -1);

        lru_cache.put(2, 6);
        assert_eq!(lru_cache.get(2), 6);
        assert_eq!(lru_cache.get(1), -1);
        lru_cache.put(1, 5);
        lru_cache.put(1, 2);
        assert_eq!(lru_cache.get(1), 2);
        assert_eq!(lru_cache.get(2), 6);
    }

    fn test3() {
        let mut lru_cache = LRUCache::new(3);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        lru_cache.put(3, 3);
        lru_cache.put(4, 4);

        assert_eq!(lru_cache.get(4), 4);
        assert_eq!(lru_cache.get(3), 3);

        assert_eq!(lru_cache.get(2), 2);
    }
}
