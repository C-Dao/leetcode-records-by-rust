/*
 * @lc app=leetcode id=328 lang=rust
 *
 * [328] Odd Even Linked List
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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_dummy_head = Box::new(ListNode {
            val: -1,
            next: None,
        });
        let mut even_dummy_head = Box::new(ListNode {
            val: -1,
            next: None,
        });

        let mut even_cur = even_dummy_head.as_mut();
        let mut odd_cur = odd_dummy_head.as_mut();

        while head.is_some() {
            let odd = match head.take() {
                Some(node) => node,
                None => break,
            };

            odd_cur.next = Some(odd);
            odd_cur = odd_cur.next.as_mut().unwrap();

            let even = match odd_cur.next.take() {
                Some(node) => node,
                None => break,
            };

            even_cur.next = Some(even);
            even_cur = even_cur.next.as_mut().unwrap();

            head = even_cur.next.take();
        }

        odd_cur.next = even_dummy_head.next;

        odd_dummy_head.next
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

    println!("{:?}", Solution::odd_even_list(Some(a)));
}
