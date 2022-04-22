/*
  234. Palindrome Linked List

  https://leetcode.com/problems/palindrome-linked-list/

  Easy
*/
use super::data_structure::list_node::ListNode;

pub struct Solution;

impl Solution {
  pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut arr = Vec::new();
    let mut head = head;
    while let Some(node) = head {
        arr.push(node.val);
        head = node.next;
    }
    let (mut p1, mut p2) = (0, arr.len().saturating_sub(1));
    while p1 < p2 {
        if arr[p1] != arr[p2] { return false; }
        p1 += 1;
        p2 -= 1;
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::data_structure::list_node::to_list;

  #[test]
  fn test_success() {
    assert!(Solution::is_palindrome(to_list(vec![1,2,3,2,1])));
  }
} 
  
