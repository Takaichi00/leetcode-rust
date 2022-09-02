// https://leetcode.com/problems/middle-of-the-linked-list/
use derive_new::new;

#[derive(new)]
pub struct Solution {}

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

////////////// ↑ ここまでコピペ不要
////////////// ↓ ここから実装をコピペする
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // https://leetcode.com/explore/featured/card/the-leetcode-beginners-guide/692/challenge-problems/4426/ を参考にする
        todo!()
    }
}


#[cfg(test)]
mod test {
    use crate::middle_of_the_linked_list_876::{ListNode, Solution};

    #[test]
    fn test_solution1() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let mut node5 = ListNode::new(5);
        node4.push(node5.clone());
        node3.push(node4.clone());
        node2.push(node3.clone());
        node1.push(node2.clone());

        let result = Solution::middle_node(Some(Box::new(node1))).unwrap();


        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let mut node5 = ListNode::new(5);

        node4.push(node5.clone());
        node3.push(node4.clone());

        assert_eq!(result, Box::new(node3));
    }
}
