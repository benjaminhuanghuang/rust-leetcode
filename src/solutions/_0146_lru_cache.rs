/*
    146. LRU Cache
    
    https://leetcode.com/problems/lru-cache/

    Medium


    Least recently used, 首先丢弃最近使用的项目
    The functions get and put must each run in O(1) average time complexity.
*/



mod Solution1{
    /*
        https://www.youtube.com/watch?v=EfjN80PyMoM
    */
    use std::{cell::RefCell, collections::HashMap, hash::HashMap, rc::Weak};
    use crate::node:: {List, Node};

    struct LRUCache {
       list: list<T>
    }


    /** 
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl LRUCache {

        fn new(capacity: i32) -> Self {
            LRU::with_capacity(10)
        }

       pub fn with_capacity(capacity: i32) -> Self {
            LRUCache {
                list: List::new(),
                map: HashMap::new(),
                capacity,
            }
        }
        
        fn get(&self, key: i32) -> i32 {
            
        }
        
        fn put(&self, key: i32, value: i32) {
            
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
    __TEST_EXTRA_USE__

    #[test]
    fn test_146() {
    }
}