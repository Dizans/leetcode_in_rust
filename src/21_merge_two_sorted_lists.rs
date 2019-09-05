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

struct Solution;

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>)
                           -> Option<Box<ListNode>> {
        match (&l1, &l2) {
            (None, None) => None,
            (None, Some(_)) => l2,
            (Some(_), None) => l1,
            (Some(i), Some(j)) => {
                let mut node;
                if i.val < j.val{
                    node = Box::new(ListNode::new(i.val));
                    node.next = Self::merge_two_lists(l1.unwrap().next, l2);
                }else{
                    node = Box::new(ListNode::new(j.val));
                    node.next = Self::merge_two_lists(l1, l2.unwrap().next);
                }
                Some(node)
            }
        }
    }
}