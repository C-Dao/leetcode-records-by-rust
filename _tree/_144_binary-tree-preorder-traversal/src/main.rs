/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
 *
 * https://leetcode.com/problems/binary-tree-preorder-traversal/description/
 *
 * algorithms
 * Easy (63.37%)
 * Likes:    6211
 * Dislikes: 163
 * Total Accepted:    1.2M
 * Total Submissions: 1.8M
 * Testcase Example:  '[1,null,2,3]'
 *
 * Given the root of a binary tree, return the preorder traversal of its nodes'
 * values.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,null,2,3]
 * Output: [1,2,3]
 *
 *
 * Example 2:
 *
 *
 * Input: root = []
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: root = [1]
 * Output: [1]
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
 *
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 *
 */
use data_structure_marcos::*;
use data_structures::*;

struct Solution {}

// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /** iteraction */
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut cur = root.clone();
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let left = cur.as_ref().unwrap().borrow().left.clone();
                ans.push(cur.as_ref().unwrap().borrow().val);
                stack.push(cur);
                cur = left;
            }

            cur = stack.pop().unwrap();
            let right = cur.as_ref().unwrap().borrow().right.clone();
            cur = right;
        }
        ans
    }
    /** recursion */
    pub fn preorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];

        Self::dfs_recursion(root, &mut ans);

        ans
    }

    pub fn dfs_recursion(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if root.is_none() {
            return;
        };

        ans.push(root.as_ref().unwrap().borrow().val);
        Self::dfs_recursion(root.as_ref().unwrap().borrow().left.clone(), ans);
        Self::dfs_recursion(root.as_ref().unwrap().borrow().right.clone(), ans);
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::preorder_traversal(binary_tree!([1, null, 2, 3])),
        [1, 2, 3]
    );

    assert_eq!(
        Solution::preorder_traversal_recursion(binary_tree!([1, null, 2, 3])),
        [1, 2, 3]
    );
}
