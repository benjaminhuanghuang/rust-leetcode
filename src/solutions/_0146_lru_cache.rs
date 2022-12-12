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
    next: Option<Rc<RefCell<LRUEntry>>>,
    prev: Option<Weak<RefCell<LRUEntry>>>,
}

impl LRUEntry {
    pub fn new(val: i32) -> Self {
        LRUEntry {
            val: val,
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

    pub fn get(&mut self, key: i32) -> i32 {
        let ptr = self.map.get_mut(&key);

        // key does not exist
        if ptr.is_none() {
            return -1;
        }

        // update by removing the current entry and inserting a new one
        let ptr = ptr.unwrap();
        let ptr = ptr.upgrade();
        match ptr {
            None => -1,
            Some(entry) => {
                let value = entry.borrow().val;
                self.remove(key, &mut entry);
                self.insert(key, value);
                value
            }
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let ptr = self.map.get_mut(&key);

        let ptr = if ptr.is_some() {
            // key existed: remove current entry and insert new entry
            self.remove(key, ptr.as_ref());
            self.insert(key, value);
        } else {
            // key does not exist, insert a new entry, check capacity
            self.dlist.push_back(value);
            if let Some(tail) = self.dlist.get_weak_tail() {
                self.map.insert(key, tail);
            }

            if self.len() > self.capacity {
                self.dlist.pop_front();
            }
        };
    }

    //-------------------------------
    // 2 utility functions
    // remove an entry from the double linked list and map
    fn remove(&mut self, key: i32, entryPtr: &mut Rc<RefCell<LRUEntry>>) {
        // 1. remove the entry from linked-list
        let (prev, next) = {
            let mut node = entryPtr.borrow_mut();
            let prev = match node.prev.take() {
                None => None,
                Some(prev) => prev.upgrade(),
            };
            let next = node.next.take();
            (prev, next)
        };

        match (prev, next) {
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            (None, Some(next)) => {
                next.borrow_mut().prev = None;
                self.head.replace(next);
            }
            (Some(prev), None) => {
                prev.borrow_mut().prev = None;
                self.tail.replace(prev);
            }
            (Some(prev), Some(next)) => {
                next.borrow_mut().prev.replace(Rc::downgrade(&prev));
                prev.borrow_mut().next.replace(next);
            }
        }

        // 2. remove entity from hashmap
        self.map.remove(&key);
    }

    // insert key-entry into hashmap and back_push entry to double linked list
    fn insert(&mut self, key: i32, val: i32) {
        let newEntry = Box::new(LRUEntry::new(val));

        if let Some(tail) = self.tail {
            // append new entry to the tail of the linked-list
            tail.next = Some(newEntry);
            newEntry.prev = Some(tail);
            self.tail = tail.next;
        } else {
            // if linked-list is empty
            self.tail = Some(newEntry);
            self.head = Some(newEntry);
        }
        self.map.insert(key, newEntry);
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
        println!("init cache");
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        println!("return 1");
        assert_eq!(lru_cache.get(1), 1); // returns 1
        println!("evict key 2");
        lru_cache.put(3, 3); // evicts key 2
        println!("return -1");
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        lru_cache.put(4, 4); // evicts key 1
        assert_eq!(lru_cache.get(1), -1); // returns -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // returns 3
        assert_eq!(lru_cache.get(4), 4); // returns 4
    }
}
