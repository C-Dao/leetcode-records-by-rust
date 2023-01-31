/*
 * @lc app=leetcode id=148 lang=rust
 *
 * [148] Sort List
 *
 * https://leetcode.com/problems/sort-list/description/
 *
 * algorithms
 * Medium (52.86%)
 * Likes:    8891
 * Dislikes: 271
 * Total Accepted:    584.3K
 * Total Submissions: 1.1M
 * Testcase Example:  '[4,2,1,3]'
 *
 * Given the head of a linked list, return the list after sorting it in
 * ascending order.
 *
 *
 * Example 1:
 *
 *
 * Input: head = [4,2,1,3]
 * Output: [1,2,3,4]
 *
 *
 * Example 2:
 *
 *
 * Input: head = [-1,5,3,4,0]
 * Output: [-1,0,3,4,5]
 *
 *
 * Example 3:
 *
 *
 * Input: head = []
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the list is in the range [0, 5 * 10^4].
 * -10^5 <= Node.val <= 10^5
 *
 *
 *
 * Follow up: Can you sort the linked list in O(n logn) time and O(1) memory
 * (i.e. constant space)?
 *
 */

use data_structure_marcos::*;
use data_structures::BoxListNode as ListNode;

struct Solution {}

// @lc code=start
impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut head_2 = Self::split(&mut head);
        let mut head_1 = head;

        head_1 = Self::sort_list(head_1);
        head_2 = Self::sort_list(head_2);

        Self::merge(head_1, head_2)
    }

    fn split(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
    fn merge(
        mut head_1: Option<Box<ListNode>>,
        mut head_2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;

        while head_1.is_some() && head_2.is_some() {
            if head_1.as_ref().unwrap().val > head_2.as_ref().unwrap().val {
                cur.next = head_2;
                head_2 = cur.next.as_mut().unwrap().next.take();
            } else {
                cur.next = head_1;
                head_1 = cur.next.as_mut().unwrap().next.take();
            }
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = if head_1.is_none() { head_2 } else { head_1 };

        dummy.next
    }
}
// @lc code=end

fn main() {
    let list_node = box_list!([4, 2, 1, 3]);
    assert_eq!(Solution::sort_list(list_node), box_list!([1, 2, 3, 4]));
}
