// https://leetcode.com/problems/middle-of-the-linked-list/
use derive_new::new;

#[derive(new)]
pub struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let node1 = ListNode::new(1);
        Some(Box::new(node1))
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
          next: None,
          val
        }
    }

    fn has_next(&self) -> bool {
        self.next.is_some()
    }
}

impl Iterator for ListNode {
    type Item = Box<ListNode>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.clone()
    }
}

impl ListNode {
    fn push(&mut self, next_node: ListNode) -> () {
        self.next = Some(Box::new(next_node));
    }
}

#[cfg(test)]
mod test {
    use crate::middle_of_the_linked_list_876::{ListNode, Solution};

    #[test]
    fn test_list_node() {
        let mut node1 = ListNode::new(1);
        let node2 = ListNode::new(2);

        node1.push(node2);

        let arg = Some(Box::new(node1));

        Solution::middle_node(None).unwrap();

    }

    #[test]
    fn test_solution1() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let mut node5 = ListNode::new(5);
        node1.push(node2);
        // node2.push(node3);
        // node3.push(node4);
        // node4.push(node5);

        let result = Solution::middle_node(Some(Box::new(node1))).unwrap();
    }
}