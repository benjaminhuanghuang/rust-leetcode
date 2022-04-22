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
  true
}
