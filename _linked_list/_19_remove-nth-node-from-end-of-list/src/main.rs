/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
 */
struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @lc code=start
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();

        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        while let Some(node) = fast.next {
            fast = node;
            slow = slow.next.as_mut().unwrap();
        }

        slow.next = slow.next.as_ref().unwrap().next.clone();

        dummy.next
    }
}
// @lc code=end

fn main() {
    let mut a = Box::new(ListNode::new(1));
    let mut b = Box::new(ListNode::new(2));
    let mut c = Box::new(ListNode::new(3));
    let mut d = Box::new(ListNode::new(4));
    let e = Box::new(ListNode::new(5));
    let n = 2;

    d.next = Some(e);
    c.next = Some(d);
    b.next = Some(c);
    a.next = Some(b);

    println!("{:?}", Solution::remove_nth_from_end(Some(a), n));
}
