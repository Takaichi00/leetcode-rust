// https://leetcode.com/problems/middle-of-the-linked-list/
use derive_new::new;

#[derive(new)]
pub struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let node1 = ListNode::new(1);
        let node2 = ListNode::new(2);
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
        // Box::new(ListNode)
        // let target = Solution::middle_node().unwrap();
        // assert_eq!(target, 6)
    }
}
