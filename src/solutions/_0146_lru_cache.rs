/*
    146. LRU Cache

    https://leetcode.com/problems/lru-cache/

    Medium


    If the number of keys exceeds the capacity from this operation, `evict` the least recently used key.
    清除最近最少使用的key

    The functions get and put must each run in O(1) average time complexity.
*/

/*
    Least Recently Used, 最近最少使用, 关键在于追踪每一个 entry 的 age, 每次淘汰最旧的那一个 key

    假如淘汰逻辑要做到 O(1) 复杂度, 需要引入一个链表 with head and tail, 每次 access 一个key时, 就delete它重新 push_back,
    而当达到容量要驱逐时, 则 pop_front

    Rust 的链表不支持根据引用删除任意元素，也没有 LinkedHashMap，需要自己实现一个
*/
use std::collections::HashMap;
use std::mem;
use std::ptr;

// Entry is either a map entry and a link-list node
pub struct LRUEntry {
  key: i32,
  val: i32,
  prev: *mut LRUEntry,
  next: *mut LRUEntry,
}

impl LRUEntry {
  pub fn new(key: i32, val: i32) -> Self {
    LRUEntry {
      key: key,
      val: val,
      prev: ptr::null_mut(),
      next: ptr::null_mut(),
    }
  }
}

pub struct LRUCache {
  map: HashMap<i32, Box<LRUEntry>>,
  cap: usize,

  // head and tail is dummy node of the double-linked-list
  head: *mut LRUEntry,
  tail: *mut LRUEntry,
}

impl LRUCache {
  pub fn new(capacity: i32) -> Self {
    let capacity = capacity as usize;
    let map = HashMap::with_capacity(capacity);
    let cache = LRUCache {
      map: map,
      cap: capacity,
      head: unsafe { Box::into_raw(Box::new(mem::uninitialized::<LRUEntry>())) },
      tail: unsafe { Box::into_raw(Box::new(mem::uninitialized::<LRUEntry>())) },
    };
    unsafe {
      (*cache.head).next = cache.tail;
      (*cache.tail).prev = cache.head;
    }

    cache
  }

  pub fn get(&mut self, key: i32) -> i32 {
    let (ptr, val) = match self.map.get_mut(&key) {
      None => (None, None),
      Some(entry) => {
        let ptr: *mut LRUEntry = &mut **entry;
        (Some(ptr), Some(unsafe { (*entry).val }))
      }
    };

    if let Some(ptr) = ptr {
      self.detach(ptr);
      self.push(ptr);
    }
    val.unwrap_or(-1)
  }

  pub fn put(&mut self, key: i32, value: i32) {
    let ptr = self.map.get_mut(&key).map(|entry| {
      let ptr: *mut LRUEntry = &mut **entry;
      ptr
    });

    match ptr {
      Some(ptr) => {
        // key already exist, update it
        unsafe { (*ptr).val = value };
        self.detach(ptr);
        self.push(ptr);
      }
      None => {
        // insert new key, cache is full, evict
        if self.map.len() == self.cap {
          let mut old_entry = self.pop().unwrap();
          old_entry.key = key;
          old_entry.val = value;
          self.push(&mut *old_entry);
          self.map.insert(key, old_entry);
        } else {
          let mut new_entry = Box::new(LRUEntry::new(key, value));
          self.push(&mut *new_entry);
          self.map.insert(key, new_entry);
        }
      }
    }
  }

  // pop() remove the entry from map, detach the entry from head of linked-list, and return it
  fn pop(&mut self) -> Option<Box<LRUEntry>> {
    let next;
    unsafe { next = (*self.head).next }
    // list is empty
    if next == self.tail {
      return None;
    }
    let key = unsafe { (*next).key };
    let mut old_entry = self.map.remove(&key).unwrap();
    self.detach(&mut *old_entry);
    Some(old_entry)
  }

  // push() pushs an entry to the tail of linked-list
  fn push(&mut self, entry: *mut LRUEntry) {
    unsafe {
      // prev <-> tail
      // prev <-> entry <-> tail
      (*entry).prev = (*self.tail).prev;
      (*entry).next = self.tail;
      (*self.tail).prev = entry;
      (*(*entry).prev).next = entry;
    }
  }

  // detach() remove an entry from the linked-list
  fn detach(&mut self, entry: *mut LRUEntry) {
    unsafe {
      // prev <-> entry <-> next
      // prev <-> next
      (*(*entry).prev).next = (*entry).next;
      (*(*entry).next).prev = (*entry).prev;
    }
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
