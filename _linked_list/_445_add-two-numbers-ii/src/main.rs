/*
 * @lc app=leetcode id=445 lang=rust
 *
 * [445] Add Two Numbers II
 *
 * https://leetcode.com/problems/add-two-numbers-ii/description/
 *
 * algorithms
 * Medium (58.97%)
 * Likes:    4154
 * Dislikes: 243
 * Total Accepted:    351.8K
 * Total Submissions: 590.8K
 * Testcase Example:  '[7,2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The most significant digit comes first and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 *
 * Example 1:
 *
 *
 * Input: l1 = [7,2,4,3], l2 = [5,6,4]
 * Output: [7,8,0,7]
 *
 *
 * Example 2:
 *
 *
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [8,0,7]
 *
 *
 * Example 3:
 *
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
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
 *
 * Follow up: Could you solve it without reversing the input lists?
 *
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let rev_l1 = Self::reverse_list(l1);
        let rev_l2 = Self::reverse_list(l2);

        let (mut cur_node, mut rev_l1_pr, mut rev_l2_pr, mut carry) = (None, &rev_l1, &rev_l2, 0);

        while rev_l1_pr.is_some() || rev_l2_pr.is_some() {
            let mut sum = rev_l1_pr
                .as_ref()
                .unwrap_or(&Box::new(ListNode::new(0)))
                .val
                + rev_l2_pr
                    .as_ref()
                    .unwrap_or(&Box::new(ListNode::new(0)))
                    .val
                + carry;

            carry = sum / 10;
            sum %= 10;

            let mut new_node = Box::new(ListNode::new(sum));
            new_node.next = cur_node;
            cur_node = Some(new_node);

            rev_l1_pr = rev_l1_pr.as_ref().map_or(&None, |ref node| &node.next);
            rev_l2_pr = rev_l2_pr.as_ref().map_or(&None, |ref node| &node.next);
        }

        if carry != 0 {
            let mut new_node = Box::new(ListNode::new(carry));
            new_node.next = cur_node;
            cur_node = Some(new_node);
        };

        cur_node
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

    fn add_two_numbers_edition_2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut s1, mut s2) = (vec![], vec![]);
        let (mut l1_pr, mut l2_pr, mut carry, mut cur_node) = (&l1, &l2, 0, None);

        while l1_pr.is_some() {
            s1.push(l1_pr.as_ref().unwrap().val);
            l1_pr = l1_pr.as_ref().map_or(&None, |ref node| &node.next);
        }

        while l2_pr.is_some() {
            s2.push(l2_pr.as_ref().unwrap().val);
            l2_pr = l2_pr.as_ref().map_or(&None, |ref node| &node.next);
        }

        while s1.len() != 0 || s2.len() != 0 {
            let mut sum = if s1.len() == 0 { 0 } else { s1.pop().unwrap() }
                + if s2.len() == 0 { 0 } else { s2.pop().unwrap() }
                + carry;
            carry = sum / 10;
            sum %= 10;
            let mut new_node = Box::new(ListNode::new(sum));
            new_node.next = cur_node;
            cur_node = Some(new_node);
        }

        if carry != 0 {
            let mut new_node = Box::new(ListNode::new(carry));
            new_node.next = cur_node;
            cur_node = Some(new_node);
        };
        cur_node
    }
}
// @lc code=end

fn main() {
    let mut a1 = Box::new(ListNode::new(7));
    let mut b = Box::new(ListNode::new(2));
    let mut c = Box::new(ListNode::new(4));
    let d = Box::new(ListNode::new(3));

    c.next = Some(d);
    b.next = Some(c);
    a1.next = Some(b);

    let mut a2 = Box::new(ListNode::new(5));
    let mut b = Box::new(ListNode::new(6));
    let c = Box::new(ListNode::new(4));

    b.next = Some(c);
    a2.next = Some(b);

    println!("{:?}", Solution::add_two_numbers(Some(a1.clone()), Some(a2.clone())));
    println!("{:?}", Solution::add_two_numbers_edition_2(Some(a1), Some(a2)));
}
