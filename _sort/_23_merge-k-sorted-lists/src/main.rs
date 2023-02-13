/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
 *
 * https://leetcode.com/problems/merge-k-sorted-lists/description/
 *
 * algorithms
 * Hard (47.59%)
 * Likes:    15088
 * Dislikes: 571
 * Total Accepted:    1.5M
 * Total Submissions: 3.1M
 * Testcase Example:  '[[1,4,5],[1,3,4],[2,6]]'
 *
 * You are given an array of k linked-lists lists, each linked-list is sorted
 * in ascending order.
 *
 * Merge all the linked-lists into one sorted linked-list and return it.
 *
 *
 * Example 1:
 *
 *
 * Input: lists = [[1,4,5],[1,3,4],[2,6]]
 * Output: [1,1,2,3,4,4,5,6]
 * Explanation: The linked-lists are:
 * [
 * ⁠ 1->4->5,
 * ⁠ 1->3->4,
 * ⁠ 2->6
 * ]
 * merging them into one sorted list:
 * 1->1->2->3->4->4->5->6
 *
 *
 * Example 2:
 *
 *
 * Input: lists = []
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: lists = [[]]
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * k == lists.length
 * 0 <= k <= 10^4
 * 0 <= lists[i].length <= 500
 * -10^4 <= lists[i][j] <= 10^4
 * lists[i] is sorted in ascending order.
 * The sum of lists[i].length will not exceed 10^4.
 *
 *
 */

struct Solution {}

use data_structure_marcos::*;
use data_structures::BoxListNode as ListNode;

// @lc code=start
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Entry {
    node: Option<Box<ListNode>>,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.node
            .as_ref()
            .unwrap()
            .val
            .cmp(&other.node.as_ref().unwrap().val)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Entry {
    fn new(node: Option<Box<ListNode>>) -> Self {
        Self { node }
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;
        let mut min_heap = BinaryHeap::new();

        for list in lists {
            if list.is_some() {
                min_heap.push(Reverse(Entry::new(list)));
            }
        }

        while !min_heap.is_empty() {
            let Reverse(least) = min_heap.pop().unwrap();
            cur.next = least.node;
            cur = cur.next.as_mut().unwrap();

            if cur.next.is_some() {
                min_heap.push(Reverse(Entry::new(cur.next.take())));
            }
        }

        dummy.next
    }

    pub fn merge_k_lists_merge_sort(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        };
        let len = lists.len();
        return Self::merge_list(&mut lists, 0, len);
    }

    fn merge_list(
        lists: &mut Vec<Option<Box<ListNode>>>,
        start: usize,
        end: usize,
    ) -> Option<Box<ListNode>> {
        if start + 1 == end {
            return lists[start].take();
        };

        let mid = (start + end) / 2;

        let head_1 = Self::merge_list(lists, start, mid);
        let head_2 = Self::merge_list(lists, mid, end);

        Self::merge(head_1, head_2)
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
    assert_eq!(
        Solution::merge_k_lists(vec![
            box_list!([1, 4, 5]),
            box_list!([1, 3, 4]),
            box_list!([2, 6])
        ]),
        box_list!([1, 1, 2, 3, 4, 4, 5, 6])
    );

    assert_eq!(
        Solution::merge_k_lists_merge_sort(vec![
            box_list!([1, 4, 5]),
            box_list!([1, 3, 4]),
            box_list!([2, 6])
        ]),
        box_list!([1, 1, 2, 3, 4, 4, 5, 6])
    );
}
