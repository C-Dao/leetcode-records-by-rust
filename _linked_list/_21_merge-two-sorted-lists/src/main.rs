/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 *
 * https://leetcode.com/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (60.95%)
 * Likes:    13998
 * Dislikes: 1259
 * Total Accepted:    2.5M
 * Total Submissions: 4M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * You are given the heads of two sorted linked lists list1 and list2.
 * 
 * Merge the two lists in a one sorted list. The list should be made by
 * splicing together the nodes of the first two lists.
 * 
 * Return the head of the merged linked list.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: list1 = [1,2,4], list2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: list1 = [], list2 = []
 * Output: []
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: list1 = [], list2 = [0]
 * Output: [0]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in both lists is in the range [0, 50].
 * -100 <= Node.val <= 100
 * Both list1 and list2 are sorted in non-decreasing order.
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur_1 = list1;
        let mut cur_2 = list2;
        let mut dummy_head_3 = Box::new(ListNode {
            val: -1,
            next: None,
        });
        let mut cur_3 = dummy_head_3.as_mut();
        while cur_1.is_some() || cur_2.is_some() {
            if cur_1.is_none() {
                cur_3.next = cur_2;
                cur_3 = cur_3.next.as_mut().unwrap();
                cur_2 = cur_3.next.take();
                continue;
            }

            if cur_2.is_none() {
                cur_3.next = cur_1;
                cur_3 = cur_3.next.as_mut().unwrap();
                cur_1 = cur_3.next.take();
                continue;
            }

            if cur_1.as_ref().unwrap().val < cur_2.as_ref().unwrap().val {
                cur_3.next = cur_1;
                cur_3 = cur_3.next.as_mut().unwrap();
                cur_1 = cur_3.next.take();
                continue;
            }

            if cur_1.as_ref().unwrap().val >= cur_2.as_ref().unwrap().val {
                cur_3.next = cur_2;
                cur_3 = cur_3.next.as_mut().unwrap();
                cur_2 = cur_3.next.take();
                continue;
            }
        }

        dummy_head_3.next
    }
}
// @lc code=end

fn main() {
    let mut a = Box::new(ListNode::new(1));
    let mut b = Box::new(ListNode::new(2));
    let c = Box::new(ListNode::new(4));

    let mut d = Box::new(ListNode::new(1));
    let mut e = Box::new(ListNode::new(3));
    let f = Box::new(ListNode::new(4));

    b.next = Some(c);
    a.next = Some(b);

    e.next = Some(f);
    d.next = Some(e);

    println!("{:?}", Solution::merge_two_lists(Some(a), Some(d)));

    println!("{:?}", Solution::merge_two_lists(None, None));

    let a = Box::new(ListNode::new(0));

    println!("{:?}", Solution::merge_two_lists(None, Some(a)));
}
