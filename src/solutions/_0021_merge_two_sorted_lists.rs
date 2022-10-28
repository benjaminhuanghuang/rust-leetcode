/*
  21. Merge Two Sorted Lists

  https://leetcode.com/problems/merge-two-sorted-lists/

  Easy
*/

use crate::util::list_node::ListNode;
pub struct Solution;
impl Solution {
  pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    if list1 == None {
      return list2;
    }
    if list2 == None {
      return list1;
    }

    let mut l1 = list1;
    let mut l2 = list2;

    let mut dummy = ListNode::new(-1);
    let mut curr = &mut dummy;

    while l1.is_some() && l2.is_some() {
      if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
        // better version: without data clone
        curr.next = l1.take();
        curr = curr.next.as_mut().unwrap();
        l1 = curr.next.take();
      } else {
        curr.next = l2.take();
        curr = curr.next.as_mut().unwrap();
        l2 = curr.next.take();
      }
    }

    if l1.is_some() {
      curr.next = l1.take();
    }
    if l2.is_some() {
      curr.next = l2.take();
    }

    dummy.next
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::merge_two_lists(None, None), None);
  }
}
