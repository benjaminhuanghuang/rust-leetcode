/*
    111. Minimum Depth of Binary Tree
    
    https://leetcode.com/problems/minimum-depth-of-binary-tree/

    Easy
*/
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};


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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_111() {
    }
}