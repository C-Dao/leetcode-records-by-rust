/*
 * @lc app=leetcode id=513 lang=rust
 *
 * [513] Find Bottom Left Tree Value
 *
 * https://leetcode.com/problems/find-bottom-left-tree-value/description/
 *
 * algorithms
 * Medium (65.70%)
 * Likes:    2664
 * Dislikes: 237
 * Total Accepted:    200.2K
 * Total Submissions: 300.7K
 * Testcase Example:  '[2,1,3]'
 *
 * Given the root of a binary tree, return the leftmost value in the last row
 * of the tree.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [2,1,3]
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,2,3,4,null,5,6,null,null,7]
 * Output: 7
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [1, 10^4].
 * -2^31 <= Node.val <= 2^31 - 1
 *
 *
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}
// @lc code=start

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue_1 = VecDeque::new();
        let mut queue_2 = VecDeque::new();

        let mut ans = root.as_ref().unwrap().borrow().val;
        queue_1.push_back(root.unwrap());

        while let Some(node) = queue_1.front() {
            if node.borrow().left.is_some() {
                let node_left_child = node.borrow().left.as_ref().unwrap().clone();
                queue_2.push_back(node_left_child);
            }

            if node.borrow().right.is_some() {
                let node_right_child = node.borrow().right.as_ref().unwrap().clone();
                queue_2.push_back(node_right_child);
            }

            queue_1.pop_front();

            if queue_1.is_empty() && queue_2.front().is_some() {
                ans = queue_2.front().unwrap().borrow().val;
                queue_1 = queue_2;
                queue_2 = VecDeque::new();
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    let mut node_1 = TreeNode::new(2);
    let node_2 = TreeNode::new(1);
    let node_3 = TreeNode::new(3);

    node_1.left = Some(Rc::new(RefCell::new(node_2)));
    node_1.right = Some(Rc::new(RefCell::new(node_3)));

    let root = Some(Rc::new(RefCell::new(node_1)));

    assert_eq!(Solution::find_bottom_left_value(root), 1);
}
