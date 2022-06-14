/*
  876. Middle of the Linked List

  https://leetcode.com/problems/middle-of-the-linked-list/

  Easy
*/
use super::data_structure::list_node::ListNode;

pub struct Solution;

impl Solution {
  pub fn middle_node_slow(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow_ptr = head.clone();
    let mut fast_ptr = head.clone();

    if head.is_some() {
      loop {
        if fast_ptr.is_some() && fast_ptr.clone().unwrap().next.is_some() {
          fast_ptr = fast_ptr.unwrap().next.unwrap().next;
          slow_ptr = slow_ptr.unwrap().next;
        } else {
          break;
        }
      }
    }

    slow_ptr
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::middle_node_slow(None), None);
  }
}
