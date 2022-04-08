/*

https://leetcode.com/problems/add-two-numbers/

Medium
*/

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  type List = Option<Box<ListNode>>;
  
  pub fn to_list(vec: Vec<i32>) -> List {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

  #[test]
  fn test_success() {
    assert_eq!(
      Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
      to_list(vec![7, 0, 8])
    );
  }
}
