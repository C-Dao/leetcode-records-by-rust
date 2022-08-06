/*
 * @lc app=leetcode id=203 lang=rust
 *
 * [203] Remove Linked List Elements
 */

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

struct Solution {}
// @lc code=start

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = dummy.as_mut();
        while let Some(ref mut temp) = cur.next {
            if temp.val == val {
                cur.next = temp.next.take();
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
// @lc code=end

fn main() {
    let mut a = Box::new(ListNode::new(1));
    let mut b = Box::new(ListNode::new(2));
    let mut c = Box::new(ListNode::new(6));
    let mut d = Box::new(ListNode::new(3));
    let mut e = Box::new(ListNode::new(4));
    let mut f = Box::new(ListNode::new(5));
    let g = Box::new(ListNode::new(6));

    f.next = Some(g);
    e.next = Some(f);
    d.next = Some(e);
    c.next = Some(d);
    b.next = Some(c);
    a.next = Some(b);

    println!("{:?}", Solution::remove_elements(Some(a), 6));
}
