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
        println!("args: {:#?}", head);
        let mut mut_head = head.clone();
        let mut list: Vec<ListNode> = Vec::new();

        while mut_head.is_some() {
            list.push(*mut_head.clone().unwrap());
            mut_head = mut_head.clone().unwrap().next;
        }

        println!("list: {:#?}", list);

        let result = list.get(list.len() / 2).unwrap().clone();

        println!("result: {:#?}", result);

        Some(Box::new(result))
    }

    pub fn middle_node_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut middle = head.clone();
        let mut end = head.clone();

        while end.clone().is_some() && end.clone().unwrap().next.is_some() {
            middle = middle.clone().unwrap().next;
            end = end.clone().unwrap().next.unwrap().next;
        }

        middle
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
        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));
        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));

        let result = Solution::middle_node(Some(Box::new(node1))).unwrap();


        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let mut node5 = ListNode::new(5);

        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));

        assert_eq!(result, Box::new(node3));
    }

    #[test]
    fn test_solution2() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let mut node5 = ListNode::new(5);
        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));
        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));

        let result = Solution::middle_node_2(Some(Box::new(node1))).unwrap();


        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let mut node5 = ListNode::new(5);

        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));

        assert_eq!(result, Box::new(node3));
    }
}
