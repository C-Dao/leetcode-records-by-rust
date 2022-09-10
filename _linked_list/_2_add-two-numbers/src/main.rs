/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (39.05%)
 * Likes:    20479
 * Dislikes: 4064
 * Total Accepted:    3M
 * Total Submissions: 7.6M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 * 
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
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
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode { val: -1, next: l1 });
        let mut cur_1 = dummy_head.as_mut();
        let mut cur_2 = l2;

        while cur_1.next.is_some() && cur_2.is_some() {
            let num_1 = cur_1.next.as_mut().unwrap().val;
            let num_2 = cur_2.as_ref().unwrap().val;
            let mod_val = (num_1 + num_2) % 10;
            let carry = (num_1 + num_2) / 10;

            cur_1.next.as_mut().unwrap().val = mod_val;

            if cur_1.next.as_mut().unwrap().next.is_some() {
                cur_1.next.as_mut().unwrap().next.as_mut().unwrap().val += carry;
            } else if carry != 0 {
                cur_1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
            };

            cur_2 = cur_2.take().unwrap().next;
            cur_1 = cur_1.next.as_mut().unwrap();
        }

        while cur_1.next.is_some() {
            let num_1 = cur_1.next.as_mut().unwrap().val;
            let mod_val = num_1 % 10;
            let carry = num_1 / 10;

            cur_1.next.as_mut().unwrap().val = mod_val;

            if cur_1.next.as_mut().unwrap().next.is_some() {
                cur_1.next.as_mut().unwrap().next.as_mut().unwrap().val += carry;
            } else if carry != 0 {
                cur_1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
            };

            cur_1 = cur_1.next.as_mut().unwrap();
        }

        if cur_2.is_some() {
            cur_1.next = cur_2.take();
        }

        dummy_head.next
    }
}
// @lc code=end

fn main() {
    let mut a = Box::new(ListNode::new(2));
    let mut b = Box::new(ListNode::new(4));
    let c = Box::new(ListNode::new(3));

    let mut d = Box::new(ListNode::new(5));
    let mut e = Box::new(ListNode::new(6));
    let f = Box::new(ListNode::new(4));

    b.next = Some(c);
    a.next = Some(b);

    e.next = Some(f);
    d.next = Some(e);

    println!("{:?}", Solution::add_two_numbers(Some(a), Some(d)));

    let a = Box::new(ListNode::new(0));
    let b = Box::new(ListNode::new(0));

    println!("{:?}", Solution::add_two_numbers(Some(a), Some(b)));
}
