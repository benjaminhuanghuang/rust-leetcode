/*
    147. Insertion Sort List

    https://leetcode.com/problems/insertion-sort-list/

    Medium
*/

use crate::util::list_node::ListNode;

pub struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

/*
    node     node --> node --> node --> node --> None
               ^
               |
              head

    sorted --> node -- None
*/
impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut sorted = Some(Box::new(ListNode::new(-1)));

        while let Some(mut node) = head {
            // cut off the first node in thr original list, which is pointed by head
            // head = node.next;
            // node.next = None;
            head = std::mem::take(&mut node.next);

            let mut p = &mut sorted;
            // find the insert position in the sorted list, which is pointed by p
            // skip the node which p.val < node.val
            // p                   is &mut Option<Box<ListNode>>,
            // p.as_ref            is Option<&Box<ListNode>>
            // p.as_ref().unwrap() is &Box<ListNode>
            while p.as_ref().unwrap().next.is_some()
                && p.as_ref().unwrap().next.as_ref().unwrap().val < node.val
            {
                p = &mut p.as_mut().unwrap().next;
            }

            // connect node with p.next
            // take() set p.next to None
            node.next = std::mem::take(&mut p.as_mut().unwrap().next);
            // connect p.next with node
            p.as_mut().unwrap().next = Some(node)
        }

        sorted.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::list_node::to_list;

    #[test]
    fn test_147() {
        assert_eq!(
            Solution::insertion_sort_list(to_list(vec![-1, 5, 3, 4, 0])),
            to_list(vec![-1, 0, 3, 4, 5])
        );
    }
}
