/*
 * @lc app=leetcode id=538 lang=rust
 *
 * [538] Convert BST to Greater Tree
 *
 * https://leetcode.com/problems/convert-bst-to-greater-tree/description/
 *
 * algorithms
 * Medium (66.71%)
 * Likes:    4638
 * Dislikes: 167
 * Total Accepted:    259.2K
 * Total Submissions: 384.1K
 * Testcase Example:  '[4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]'
 *
 * Given the root of a Binary Search Tree (BST), convert it to a Greater Tree
 * such that every key of the original BST is changed to the original key plus
 * the sum of all keys greater than the original key in BST.
 *
 * As a reminder, a binary search tree is a tree that satisfies these
 * constraints:
 *
 *
 * The left subtree of a node contains only nodes with keys less than the
 * node's key.
 * The right subtree of a node contains only nodes with keys greater than the
 * node's key.
 * Both the left and right subtrees must also be binary search trees.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
 * Output: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
 *
 *
 * Example 2:
 *
 *
 * Input: root = [0,null,1]
 * Output: [1,null,1]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 10^4].
 * -10^4 <= Node.val <= 10^4
 * All the values in the tree are unique.
 * root is guaranteed to be a valid binary search tree.
 *
 *
 *
 * Note: This question is the same as 1038:
 * https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/
 *
 */

use data_structure_marcos::*;
use data_structures::*;

struct Solution {}
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /** in-order traversal, iteraction */
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = root.clone();
        let mut stack = vec![];
        let mut sum = 0;
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let right = cur.as_ref().unwrap().borrow().right.clone();
                stack.push(cur);
                cur = right;
            }

            cur = stack.pop().unwrap();

            sum += cur.as_ref().unwrap().borrow().val;
            cur.as_mut().unwrap().borrow_mut().val = sum;

            let left = cur.as_ref().unwrap().borrow().left.clone();
            cur = left;
        }

        root
    }

    /** in-order traversal, recursion */
    pub fn convert_bst_recursion(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        Self::dfs_recursion(root.clone(), &mut sum);
        root
    }

    pub fn dfs_recursion(mut root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if root.is_none() {
            return;
        }

        Self::dfs_recursion(root.as_ref().unwrap().borrow().right.clone(), sum);
        *sum += root.as_ref().unwrap().borrow().val;
        root.as_mut().unwrap().borrow_mut().val = *sum;
        Self::dfs_recursion(root.as_ref().unwrap().borrow().left.clone(), sum);
    }
}
// @lc code=end

fn main() {
    let tree_0 = binary_tree!([4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8]);
    let tree_1 = binary_tree!([4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8]);
    let expected_tree =
        binary_tree!([30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8]);

    assert_eq!(Solution::convert_bst(tree_0), expected_tree);
    assert_eq!(Solution::convert_bst_recursion(tree_1), expected_tree);
}
