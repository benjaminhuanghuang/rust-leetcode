/*
  1290. Convert Binary Number in a Linked List to Integer

  https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/

  Easy
*/
use super::data_structure::list_node::ListNode;

pub struct Solution;

impl Solution {
  pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut curr = &head;
    let mut result = 0;
    while let Some(node) = curr {
      result = (result << 1) + node.val;
      curr = &node.next;
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::get_decimal_value(None), 0);
  }
}
