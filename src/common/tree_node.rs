use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
  use std::collections::VecDeque;
  let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
  let mut queue = VecDeque::new();
  queue.push_back(head.as_ref().unwrap().clone());

  for children in vec[1..].chunks(2) {
    let parent = queue.pop_front().unwrap();
    if let Some(v) = children[0] {
      parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
      queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
    }
    if children.len() > 1 {
      if let Some(v) = children[1] {
        parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
      }
    }
  }
  head
}

#[macro_export]
macro_rules! tree {
  // using pattern matching on the input
  // case1: no arguments are passed to the macro
  () => {
      None
  };
  // case2: match any number of comma-separated expressions
  // each of these expressions is captured as $e.
  ($($e:expr),*) => {
      {
          //stringifies them
          // The $( ... ),* syntax is a repetition operator in Rust macros.
          // stringify!($e) will be repeated for each captured $e expression, separated by commas.
          let vec = vec![$(stringify!($e)), *];
          // pass items to integer
          let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
          to_tree(vec)
      }
  };
  // handles the case where the expressions are followed by a trailing comma.
  // removes the trailing comma (by calling tree![$($e),*]) and passes it to the second pattern
  ($($e:expr,)*) => {(tree![$($e),*])};
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tree_macro() {
    let tree = tree![1, 2, 3, 4, 5, 6, 7];

    let expected_tree = Some(Rc::new(RefCell::new(TreeNode {
      val: 1,
      left: Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 4,
          left: None,
          right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 5,
          left: None,
          right: None,
        }))),
      }))),
      right: Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 6,
          left: None,
          right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 7,
          left: None,
          right: None,
        }))),
      }))),
    })));

    assert_eq!(tree, expected_tree);
  }

  /*
      3
     / \
    9   20
       /  \
      15   7
  */
  #[test]
  fn test_tree_macro_with_null() {
    let tree: Option<Rc<RefCell<TreeNode>>> = tree![3, 9, 20, hh, null, 15, 7];
    // Expected tree structure
    let expected_tree = Some(Rc::new(RefCell::new(TreeNode {
      val: 3,
      left: Some(Rc::new(RefCell::new(TreeNode {
        val: 9,
        left: None,
        right: None,
      }))),
      right: Some(Rc::new(RefCell::new(TreeNode {
        val: 20,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 15,
          left: None,
          right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 7,
          left: None,
          right: None,
        }))),
      }))),
    })));

    assert_eq!(tree, expected_tree);
  }
}
