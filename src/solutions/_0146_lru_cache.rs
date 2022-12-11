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

// Entry is either a map entry and a link-list node
pub struct LRUEntry {
  key: i32,
  val: i32,
  prev: Option<Box<LRUEntry>>,
  next: Option<Box<LRUEntry>>,
}

impl LRUEntry {
  pub fn new(key: i32, val: i32) -> Self {
    LRUEntry {
      key: key,
      val: val,
      prev: None,
      next: None,
    }
  }
}

pub struct LRUCache {
  capacity: usize,
  length: usize,
  map: HashMap<i32, Box<LRUEntry>>,
  head: Option<Box<LRUEntry>>,
  tail: Option<Box<LRUEntry>>,
}

impl LRUCache {
  pub fn new(capacity: i32) -> Self {
    let cap = capacity as usize;
    Self {
      map: HashMap::with_capacity(cap),
      length: 0,
      capacity: cap,
      head: None,
      tail: None,
    }
  }

  pub fn get(&mut self, key: i32) -> i32 {
    if let Some(entry) = self.map.get(&key) {
      self.remove(entry);
      self.insert(entry.key, entry.val);
      return entry.val;
    }
    -1
  }

  pub fn put(&mut self, key: i32, value: i32) {
    match self.map.get(&key) {
      // key existed
      Some(entry) => {
        self.remove(entry);
        self.insert(key, value);
      }
      // new key
      None => {
        if self.length >= self.capacity {
          self.remove(self.head);
          self.insert(key, value);
        } else {
          self.insert(key, value);
          self.length += 1;
        }
      }
    }
  }

  pub fn put2(&mut self, key: i32, value: i32) {
    // if key exists, remove head and push_back an entity with new value
    if let Some(entry) = self.map.get_mut(&key) {
      self.remove(entry);
      self.insert(key, value);
      return;
    }

    // when key does not exist
    if self.length >= self.capacity {
      self.remove(self.head.as_ref());
      self.insert(key, value);
    } else {
      self.insert(key, value);
      self.length += 1;
    }
  }
  //-------------------------------
  // 2 utility functions
  // remove an entry from the linked-list and map
  fn remove(&mut self, entry: &LRUEntry) {
    // 1. remove the entry from linked-list
    let prev = entry.prev;
    let next = entry.next;
    if let Some(prev) = entry.prev {
      prev.next = next;
    }
    if let Some(next) = entry.next {
      next.prev = prev;
    }

    /* js code:
    if self.head == entry {
      self.head = next;
    }
    if self.tail == entry {
      self.tail = prev;
    }
    */
    if let Some(head) = self.head {
      if head == entry {
        self.head = next;
      }
    }

    // 2. remove entity from hashmap
    self.map.remove(&entry.key);
  }

  // insert key-val into hashmap and push an entry to the tail of linked-list
  fn insert(&mut self, key: i32, val: i32) {
    let newEntry = Box::new(LRUEntry::new(key, val));

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
