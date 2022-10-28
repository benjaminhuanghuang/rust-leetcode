/*
    111. Minimum Depth of Binary Tree

    https://leetcode.com/problems/minimum-depth-of-binary-tree/

    Easy
*/

use super::util::tree_node::TreeNode;

pub struct Solution;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
  pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
      return 0;
    }

    let mut deq = VecDeque::new();
    deq.push_back((1, root.clone()));

    while !deq.is_empty() {
      if let Some((level, Some(node))) = deq.pop_front() {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
          return level;
        }
        deq.push_back((level + 1, node.borrow().left.clone()));
        deq.push_back((level + 1, node.borrow().right.clone()));
      }
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_111() {
    assert_eq!(Solution::min_depth(tree![3, 9, 20, null, null, 15, 7]), 2);
  }
}
