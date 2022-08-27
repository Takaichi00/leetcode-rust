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

        // println!("{:#}", head.clone().unwrap().val);
        // println!("{:#}", head.clone().unwrap().next.unwrap().val);

        let mut node5 = head.unwrap().get_last_node();
        println!("node5: {:#?}", node5);
        // next が None の ListNode を new して返す を 3回する
        // → None になるまで ListNode を追って、None の ListNode を返す

        // next が None の ListNode の 2つ上の ListNode 以外は pop する

        // 1,2,3,4,5
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let mut node5 = ListNode::new(5);
        node4.push(node5.clone());
        node3.push(node4.clone());
        Some(Box::new(node3))
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

    fn has_next(&self) -> bool {
        return self.next.is_some()
    }

    fn has_next_2(&self) -> bool {

        match self.clone().next.clone() {
            Some(next_list_node) => {
                match next_list_node.clone().next.clone() {
                    Some(_) => {
                        true
                    },
                    None => {
                        false
                    }
                }
            },
            None => {
                false
            }
        }

        //
        // if self.next.is_none() {
        //     return false;
        // }
        //
        // if self.next.unwrap().next.is_none() {
        //     return false
        // }
        //
        // true
    }

    fn get_last_node(&self) -> ListNode {

        println!("self: {:#?}", self);

        if self.has_next_2() {
            return self.clone().next.clone().unwrap().get_last_node();
        }
        // 2個先が None だったら、2個先の self を返して、1個先の node の next を none にする
        *self.next.clone().unwrap().next.clone().unwrap()
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
