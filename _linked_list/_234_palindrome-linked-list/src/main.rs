/*
 * @lc app=leetcode id=234 lang=rust
 *
 * [234] Palindrome Linked List
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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let mut rev: Option<Box<ListNode>> = None;
        let mut _nxt: Option<Box<ListNode>> = None;

        // half to half
        while head.is_some() {
            if rev == head || rev == head.as_ref().unwrap().next {
                return true;
            }

            _nxt = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = rev;
            rev = head;
            head = _nxt;
        }

        false
    }
    pub fn is_palindrome_by_backtrack(head: Option<Box<ListNode>>) -> bool {
        let mut cur = head.clone();
        return Self::backtrace_check(head.as_ref(), cur.as_mut().unwrap());
    }

    fn backtrace_check(head: Option<&Box<ListNode>>, &mut cur: &mut Box<ListNode>) -> bool {
        if head.is_none() {
            return true;
        }
        let check_bool = Self::backtrace_check(head.unwrap().next.as_ref(), &mut cur)
            && (cur.val == head.unwrap().val);
        cur = cur.next.as_mut().unwrap();
        check_bool
    }
}
// @lc code=end

fn main() {
    let mut a = Box::new(ListNode::new(1));
    let mut b = Box::new(ListNode::new(2));
    let mut c = Box::new(ListNode::new(2));
    let mut d = Box::new(ListNode::new(1));

    c.next = Some(d);
    b.next = Some(c);
    a.next = Some(b);

    assert_eq!(Solution::is_palindrome_by_backtrack(Some(a)), true);
}
