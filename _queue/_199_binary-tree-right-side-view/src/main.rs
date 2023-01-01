/*
 * @lc app=leetcode id=199 lang=rust
 *
 * [199] Binary Tree Right Side View
 *
 * https://leetcode.com/problems/binary-tree-right-side-view/description/
 *
 * algorithms
 * Medium (60.56%)
 * Likes:    9131
 * Dislikes: 544
 * Total Accepted:    879.2K
 * Total Submissions: 1.4M
 * Testcase Example:  '[1,2,3,null,5,null,4]'
 *
 * Given the root of a binary tree, imagine yourself standing on the right side
 * of it, return the values of the nodes you can see ordered from top to
 * bottom.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,3,null,5,null,4]
 * Output: [1,3,4]
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,null,3]
 * Output: [1,3]
 *
 *
 * Example 3:
 *
 *
 * Input: root = []
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 100].
 * -100 <= Node.val <= 100
 *
 *
 */
struct Solution {}

// @lc code=start
use data_structure_marcos::*;
use data_structures::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue_1 = VecDeque::new();
        let mut queue_2 = VecDeque::new();
        let mut ans = vec![];

        if root.is_some() {
            queue_1.push_back(root.unwrap());
        }

        while let Some(ref node) = queue_1.front() {
            let node_val = node.borrow().val;

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
                ans.push(node_val);
                queue_1 = queue_2;
                queue_2 = VecDeque::new();
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    let root = binary_tree!([1, 2, 3]);

    assert_eq!(Solution::right_side_view(root), [1, 3]);
}
