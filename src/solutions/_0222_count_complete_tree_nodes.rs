/*
    222. Count Complete Tree Nodes

    https://leetcode.com/problems/count-complete-tree-nodes/

    Medium
*/

use crate::util::tree_node::{to_tree, TreeNode};

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
use std::rc::Rc;
impl Solution {
  pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_222() {}
}
