/*
  206. Reverse Linked List

  https://leetcode.com/problems/reverse-linked-list/

  Easy
*/
use crate::util::list_node::ListNode;

pub struct Solution;

impl Solution {
  pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut reversed = None;

    while let Some(mut node) = head {
      head = node.next;
      node.next = reversed;
      reversed = Some(node);
    }

    reversed
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::util::list_node::to_list;

  #[test]
  fn test_206() {
    assert_eq!(
      Solution::reverse_list(to_list(vec![1, 2, 3])),
      to_list(vec![3, 2, 1])
    );
  }
}
