/*
 * @lc app=leetcode id=515 lang=rust
 *
 * [515] Find Largest Value in Each Tree Row
 *
 * https://leetcode.com/problems/find-largest-value-in-each-tree-row/description/
 *
 * algorithms
 * Medium (64.55%)
 * Likes:    2468
 * Dislikes: 93
 * Total Accepted:    216.7K
 * Total Submissions: 335.4K
 * Testcase Example:  '[1,3,2,5,3,null,9]'
 *
 * Given the root of a binary tree, return an array of the largest value in
 * each row of the tree (0-indexed).
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,3,2,5,3,null,9]
 * Output: [1,3,9]
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,2,3]
 * Output: [1,3]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree will be in the range [0, 10^4].
 * -2^31 <= Node.val <= 2^31 - 1
 *
 *
 */

struct Solution {}

use data_structure_marcos::*;
use data_structures::*;
use std::cell::RefCell;
use std::rc::Rc;

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue_1 = VecDeque::new();
        let mut queue_2 = VecDeque::new();
        let mut ans = vec![];
        let mut max_value = i32::MIN;

        if root.is_some() {
            queue_1.push_back(root.unwrap());
        }

        while let Some(ref node) = queue_1.front() {
            max_value = i32::max(max_value, node.borrow().val);

            if node.borrow().left.is_some() {
                let node_left_child = node.borrow().left.as_ref().unwrap().clone();
                queue_2.push_back(node_left_child);
            }

            if node.borrow().right.is_some() {
                let node_right_child = node.borrow().right.as_ref().unwrap().clone();
                queue_2.push_back(node_right_child);
            }

            queue_1.pop_front();

            if queue_1.is_empty() {
                queue_1 = queue_2;
                queue_2 = VecDeque::new();
                ans.push(max_value);
                max_value = i32::MIN;
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    let root = binary_tree!(1, 2, 3);

    assert_eq!(Solution::largest_values(root), [1, 3]);
}
