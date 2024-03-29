/*
 * @lc app=leetcode id=234 lang=rust
 *
 * [234] Palindrome Linked List
 *
 * https://leetcode.com/problems/palindrome-linked-list/description/
 *
 * algorithms
 * Easy (47.40%)
 * Likes:    10215
 * Dislikes: 616
 * Total Accepted:    1.1M
 * Total Submissions: 2.3M
 * Testcase Example:  '[1,2,2,1]'
 *
 * Given the head of a singly linked list, return true if it is a
 * palindrome.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: head = [1,2,2,1]
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: head = [1,2]
 * Output: false
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the list is in the range [1, 10^5].
 * 0 <= Node.val <= 9
 * 
 * 
 * 
 * Follow up: Could you do it in O(n) time and O(1) space?
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

pub struct ListIter<'a> {
    node: &'a Option<Box<ListNode>>,
}

impl<'a> Iterator for ListIter<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let val = self.node.as_ref()?.val;
        self.node = &self.node.as_ref()?.next;
        Some(val)
    }
}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let _iter = ListIter { node: &head };
        let val_vec: Vec<i32> = _iter.collect();
        val_vec.iter().rev().eq(val_vec.iter())
    }

    pub fn is_palindrome_by_backtrack(head: Option<Box<ListNode>>) -> bool {
        let mut dummy_cur = head.as_ref().unwrap();
        return Self::backtrace_check(head.as_ref(), &mut dummy_cur);
    }

    fn backtrace_check(head: Option<&Box<ListNode>>, dummy_cur: &mut &Box<ListNode>) -> bool {
        if head.is_none() {
            return true;
        }
        let checked = if Self::backtrace_check(head.unwrap().next.as_ref(), dummy_cur) {
            dummy_cur.val == head.unwrap().val
        } else {
            false
        };
        if dummy_cur.next.is_some() {
            *dummy_cur = dummy_cur.next.as_ref().unwrap();
        }
        checked
    }
}
// @lc code=end

fn main() {
    let mut a = Box::new(ListNode::new(1));
    let mut b = Box::new(ListNode::new(2));
    let mut c = Box::new(ListNode::new(2));
    let d = Box::new(ListNode::new(1));

    c.next = Some(d);
    b.next = Some(c);
    a.next = Some(b);

    assert_eq!(Solution::is_palindrome_by_backtrack(Some(a.clone())), true);
    assert_eq!(Solution::is_palindrome(Some(a.clone())), true);
}
