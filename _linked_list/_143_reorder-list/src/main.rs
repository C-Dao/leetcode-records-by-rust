/*
 * @lc app=leetcode id=143 lang=rust
 *
 * [143] Reorder List
 *
 * https://leetcode.com/problems/reorder-list/description/
 *
 * algorithms
 * Medium (49.19%)
 * Likes:    7746
 * Dislikes: 267
 * Total Accepted:    606.6K
 * Total Submissions: 1.2M
 * Testcase Example:  '[1,2,3,4]'
 *
 * You are given the head of a singly linked-list. The list can be represented
 * as:
 *
 *
 * L0 → L1 → … → Ln - 1 → Ln
 *
 *
 * Reorder the list to be on the following form:
 *
 *
 * L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
 *
 *
 * You may not modify the values in the list's nodes. Only nodes themselves may
 * be changed.
 *
 *
 * Example 1:
 *
 *
 * Input: head = [1,2,3,4]
 * Output: [1,4,2,3]
 *
 *
 * Example 2:
 *
 *
 * Input: head = [1,2,3,4,5]
 * Output: [1,5,2,4,3]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the list is in the range [1, 5 * 10^4].
 * 1 <= Node.val <= 1000
 *
 *
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

use std::collections::VecDeque;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut deque = VecDeque::new();
        let mut head_ptr = head.take();

        while let Some(mut node) = head_ptr {
            head_ptr = node.next.take();
            deque.push_back(node)
        }

        let mut dummy = Some(Box::new(ListNode::new(0)));

        let mut head_ptr = dummy.as_mut();

        while deque.len() > 0 {
            head_ptr.as_mut().unwrap().next = deque.pop_front();
            head_ptr = head_ptr.unwrap().next.as_mut();
            head_ptr.as_mut().unwrap().next = deque.pop_back();
            head_ptr = head_ptr.unwrap().next.as_mut();
        }

        *head = dummy.unwrap().next;
    }
    pub fn reorder_list_edition2(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        };

        let mut l2 = Self::split_node(head);
        let l1 = head;

        l2 = Self::reverse_list(l2);
        Self::merge_list(l1, l2);
    }

    fn split_node(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut pr = head.as_ref();

        while let Some(node) = pr {
            len += 1;
            pr = node.next.as_ref();
        }
        let mut mid = head.as_mut();
        for _ in 1..(len + 1) / 2 {
            let next = mid.unwrap().next.as_mut();
            mid = next;
        }

        mid.unwrap().next.take()
    }

    fn merge_list(l1: &mut Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) {
        let mut tail = l1;

        while tail.is_some() && l2.is_some() {
            let tail_next = tail.as_mut().unwrap().next.take();
            let l2_next = l2.as_mut().unwrap().next.take();

            l2.as_mut().unwrap().next = tail_next;
            tail.as_mut().unwrap().next = l2;
            l2 = l2_next;
            tail = &mut tail.as_mut().unwrap().next.as_mut().unwrap().next;
        }
    }

    fn reverse_list(l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut cur) = (None, l);

        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node);
        }

        prev
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
    let mut some_a = Some(a);
    Solution::reorder_list(&mut some_a);
    Solution::reorder_list_edition2(&mut some_a);
}
