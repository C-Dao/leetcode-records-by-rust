/*
 * @lc app=leetcode id=105 lang=rust
 *
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 *
 * https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/
 *
 * algorithms
 * Medium (58.53%)
 * Likes:    12783
 * Dislikes: 377
 * Total Accepted:    966.5K
 * Total Submissions: 1.6M
 * Testcase Example:  '[3,9,20,15,7]\n[9,3,15,20,7]'
 *
 * Given two integer arrays preorder and inorder where preorder is the preorder
 * traversal of a binary tree and inorder is the inorder traversal of the same
 * tree, construct and return the binary tree.
 *
 *
 * Example 1:
 *
 *
 * Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
 * Output: [3,9,20,null,null,15,7]
 *
 *
 * Example 2:
 *
 *
 * Input: preorder = [-1], inorder = [-1]
 * Output: [-1]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= preorder.length <= 3000
 * inorder.length == preorder.length
 * -3000 <= preorder[i], inorder[i] <= 3000
 * preorder and inorder consist of unique values.
 * Each value of inorder also appears in preorder.
 * preorder is guaranteed to be the preorder traversal of the tree.
 * inorder is guaranteed to be the inorder traversal of the tree.
 *
 *
 */

use data_structure_marcos::*;
use data_structures::*;

struct Solution {}

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        };

        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let mut stack = vec![root.clone()];
        let mut inorder_index = 0;

        for i in 1..preorder.len() {
            let preorder_val = preorder[i];
            let mut node = stack.last().unwrap().clone();
            if node.borrow().val != inorder[inorder_index] {
                let left = Rc::new(RefCell::new(TreeNode::new(preorder_val)));
                stack.push(left.clone());
                node.borrow_mut().left = Some(left);
            } else {
                while !stack.is_empty()
                    && stack.last().unwrap().borrow().val == inorder[inorder_index]
                {
                    node = stack.pop().unwrap();
                    inorder_index += 1;
                }
                let right = Rc::new(RefCell::new(TreeNode::new(preorder_val)));
                stack.push(right.clone());
                node.borrow_mut().right = Some(right);
            }
        }

        Some(root)
    }
}
// @lc code=end

fn main() {
    let root = binary_tree!([3, 9, 20, null, null, 15, 7]);

    assert_eq!(
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
        root
    );
}
