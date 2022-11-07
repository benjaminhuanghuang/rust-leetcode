/*
    1302. Deepest Leaves Sum

    https://leetcode.com/problems/deepest-leaves-sum/

    Medium
*/

use crate::util::tree_node::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

/*
  dfs(root, curr_level)
*/
impl Solution {
  pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let maxdepth = 0_usize;
    let mut aux = vec![0; 10001];
    Solution::bfs(root, 0, &mut aux);

    let mut res = 0;
    for n in aux {
      if n > 0 {
        res = n;
      }
    }
    res
  }

  pub fn bfs(root: Option<Rc<RefCell<TreeNode>>>, maxdepth: usize, aux: &mut Vec<i32>) -> i32 {
    if let Some(root) = root {
      aux[maxdepth] += root.borrow().val;
      Solution::bfs(root.borrow().left.clone(), maxdepth + 1, aux);
      Solution::bfs(root.borrow().right.clone(), maxdepth + 1, aux);
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{tree, util::tree_node::to_tree};

  #[test]
  fn test_1302() {
    assert_eq!(
      Solution::deepest_leaves_sum(tree![1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8]),
      15
    );

    assert_eq!(
      Solution::deepest_leaves_sum(tree![
        6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5
      ]),
      19
    );
  }
}
