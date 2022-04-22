// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

type List = Option<Box<ListNode>>;

// to_list(vec![5, 6, 4])
pub fn to_list(vec: Vec<i32>) -> List {
  let mut current = None;
  for &v in vec.iter().rev() {
    let mut node = ListNode::new(v);
    node.next = current;
    current = Some(Box::new(node));
  }
  current
}

pub fn list_equal(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> bool {
  let mut l1 = l1;
  let mut l2 = l2;
  while l1.is_some() && l2.is_some() {
    let node1 = l1.unwrap();
    let node2 = l2.unwrap();
    if node1.val != node2.val { return false; }

    l1 = node1.next;
    l2 = node2.next;
  }

  l1.is_none() && l1.is_none()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_list_equal() {
    assert!(list_equal(to_list(vec![3, 2, 1]), to_list(vec![3, 2, 1])));
  }
}
