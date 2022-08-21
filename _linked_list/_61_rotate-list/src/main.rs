/*
 * @lc app=leetcode id=61 lang=rust
 *
 * [61] Rotate List
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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut temp_head = head.as_ref();
        while let Some(node) = temp_head {
            len += 1;
            temp_head = node.next.as_ref();
        }

        if len == 0 {
            return None;
        }

        let k = k.rem_euclid(len as i32);

        if k == 0 {
            return head;
        }

        let mut node = head.as_mut().unwrap();

        for _ in 0..len - k - 1 {
            node = node.next.as_mut().unwrap();
        }

        let mut new_head = node.next.take().unwrap();
        let mut new_tail = new_head.as_mut();

        for _ in 0..k - 1 {
            new_tail = new_tail.next.as_mut().unwrap();
        }

        new_tail.next = head;
        Some(new_head)
    }
}
// @lc code=end

fn main() {
    let mut a = Box::new(ListNode::new(1));
    let mut b = Box::new(ListNode::new(2));
    let mut c = Box::new(ListNode::new(3));
    let mut d = Box::new(ListNode::new(4));
    let e = Box::new(ListNode::new(5));

    d.next = Some(e);
    c.next = Some(d);
    b.next = Some(c);
    a.next = Some(b);

    println!("{:?}", Solution::rotate_right(Some(a), 2));
}
