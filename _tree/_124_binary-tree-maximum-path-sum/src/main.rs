/*
 * @lc app=leetcode id=124 lang=rust
 *
 * [124] Binary Tree Maximum Path Sum
 *
 * https://leetcode.com/problems/binary-tree-maximum-path-sum/description/
 *
 * algorithms
 * Hard (37.95%)
 * Likes:    13155
 * Dislikes: 617
 * Total Accepted:    918.9K
 * Total Submissions: 2.4M
 * Testcase Example:  '[1,2,3]'
 *
 * A path in a binary tree is a sequence of nodes where each pair of adjacent
 * nodes in the sequence has an edge connecting them. A node can only appear in
 * the sequence at most once. Note that the path does not need to pass through
 * the root.
 *
 * The path sum of a path is the sum of the node's values in the path.
 *
 * Given the root of a binary tree, return the maximum path sum of any
 * non-empty path.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,3]
 * Output: 6
 * Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 =
 * 6.
 *
 *
 * Example 2:
 *
 *
 * Input: root = [-10,9,20,null,null,15,7]
 * Output: 42
 * Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 +
 * 7 = 42.
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [1, 3 * 10^4].
 * -1000 <= Node.val <= 1000
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
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs_recursion(root).1
    }

    /** pass-order traversal */
    fn dfs_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, i32::MIN);
        };

        let (left_max_path, left_max_sum) =
            Self::dfs_recursion(root.as_ref().unwrap().borrow().left.clone());
        let (right_max_path, right_max_sum) =
            Self::dfs_recursion(root.as_ref().unwrap().borrow().right.clone());

        let max_path = i32::max(
            0,
            root.as_ref().unwrap().borrow().val + i32::max(left_max_path, right_max_path),
        );
        let max_sum = i32::max(
            i32::max(left_max_sum, right_max_sum),
            root.as_ref().unwrap().borrow().val + left_max_path + right_max_path,
        );

        (max_path, max_sum)
    }
}
// @lc code=end

fn main() {
    let mut root_0 = TreeNode::new(1);
    let node_1 = TreeNode::new(2);
    let node_2 = TreeNode::new(3);

    root_0.right = Some(Rc::new(RefCell::new(node_2)));
    root_0.left = Some(Rc::new(RefCell::new(node_1)));

    assert_eq!(
        Solution::max_path_sum(Some(Rc::new(RefCell::new(root_0)))),
        6
    );
}
