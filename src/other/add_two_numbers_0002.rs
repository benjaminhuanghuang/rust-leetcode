/*
  2. Add Two Numbers

  https://leetcode.com/problems/add-two-numbers/

  Medium
*/

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = &l1;
    let mut l2 = &l2;

    let mut carry = 0;
    let mut result = None;
    let mut cur = &mut result;

    while l1.is_some() || l2.is_some() || carry != 0 {
      let mut sum = carry;
      if let Some(node) = l1 {
        sum += node.val;
        l1 = &node.next;
      }
      if let Some(node) = l2 {
        sum += node.val;
        l2 = &node.next;
      }
      carry = sum / 10;
      *cur = Some(Box::new(ListNode::new(sum % 10)));
      cur = &mut cur.as_mut().unwrap().next;
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::list_node::to_list;

  #[test]
  fn test_add_two_numbers_success() {
    assert_eq!(
      Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
      to_list(vec![7, 0, 8])
    );
  }
}
