/*
    147. Insertion Sort List

    https://leetcode.com/problems/insertion-sort-list/

    Medium
*/

use crate::util::list_node::ListNode;

pub struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
  pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;

    let mut sorted = Some(Box::new(ListNode::new(-1)));

    while let Some(mut node) = head {
      head = node.next;
      node.next = None;

      let mut p = &mut sorted;
      while p.as_ref().unwrap().next.is_some()
        && p.as_ref().unwrap().next.as_ref().unwrap().val < node.val
      {
        p = &mut p.as_mut().unwrap().next;
      }

      node.next = std::mem::take(&mut p.as_mut().unwrap().next);

      p.as_mut().unwrap().next = Some(node)
    }

    sorted.unwrap().next
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::util::list_node::to_list;

  #[test]
  fn test_147() {
    assert_eq!(
      Solution::insertion_sort_list(to_list(vec![-1, 5, 3, 4, 0])),
      to_list(vec![-1, 0, 3, 4, 5])
    );
  }
}
