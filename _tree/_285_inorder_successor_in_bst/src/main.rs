/*
 * @lc app=leetcode id=285 lang=rust
 *
 * [285] Inorder Successor in BST
 *
 * https://leetcode.com/problems/inorder-successor-in-bst/description/
 *
 * algorithms
 * Medium
 * Likes:
 * Dislikes:
 * Total Accepted:
 * Total Submissions:
 * Testcase Example:  '[2,1,3] 1'
 *
 * Given a binary search tree and a node in it, find the in-order successor of that node in the BST.
 * The successor of a node p is the node with the smallest key greater than p.val.
 *
 * Example 1:
 *
 *
 * Input: root = [2,1,3] p = 1
 * Output: 2
 * Explanation: 1's in-order successor node is 2. Note that both p and the return value is of TreeNode type.
 *
 *
 * Example 2:
 *
 * Input: root = [5,3,6,2,4,null,null,1], p = 6
 * Output: null
 * Explanation: There is no in-order successor of the current node, so the answer is null.
 *
 *
 * Constraints:
 *
 * The number of nodes in the tree is in the range [1, 104].
 * -10^5 <= Node.val <= 10^5
 * The value of each node in the tree is guaranteed to be unique.
 *
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
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        let mut cur = root.clone();
        let mut found = false;

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let left = cur.as_ref().unwrap().borrow().left.clone();
                stack.push(cur.clone());
                cur = left;
            }

            cur = stack.pop().unwrap();

            if found {
                break;
            } else if p.as_ref().unwrap().borrow().val == cur.as_ref().unwrap().borrow().val {
                found = true;
            }

            let right = cur.as_ref().unwrap().borrow().right.clone();
            cur = right;
        }

        cur
    }

    /** in-order traversal, recursion */
    pub fn inorder_successor_recusion(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = None;
        let mut prev = None;
        Self::dfs_traversal_recusion(root, p, &mut prev, &mut ans);
        ans
    }

    pub fn dfs_traversal_recusion(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        ans: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if root.is_none() {
            return;
        }

        Self::dfs_traversal_recusion(
            root.as_ref().unwrap().borrow().left.clone(),
            p.clone(),
            prev,
            ans,
        );

        if prev.is_some() && prev.as_ref().unwrap().borrow().val == p.as_ref().unwrap().borrow().val
        {
            *ans = root.clone();
        }

        *prev = root.clone();

        Self::dfs_traversal_recusion(
            root.as_ref().unwrap().borrow().right.clone(),
            p.clone(),
            prev,
            ans,
        );
    }

    pub fn inorder_successor_edition_2(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = root.clone();
        let mut ans = None;

        while cur.is_some() {
            if cur.as_ref().unwrap().borrow().val > p.as_ref().unwrap().borrow().val {
                let left = cur.as_ref().unwrap().borrow().left.clone();
                ans = cur;
                cur = left;
            } else {
                let right = cur.as_ref().unwrap().borrow().right.clone();
                cur = right;
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    let tree = binary_tree!([2, 1, 3]);

    assert_eq!(
        Solution::inorder_successor_recusion(
            tree.clone(),
            Some(Rc::new(RefCell::new(TreeNode::new(1))))
        ),
        tree.clone()
    );
    assert_eq!(
        Solution::inorder_successor_edition_2(
            tree.clone(),
            Some(Rc::new(RefCell::new(TreeNode::new(1))))
        ),
        tree.clone()
    );
    assert_eq!(
        Solution::inorder_successor(tree.clone(), Some(Rc::new(RefCell::new(TreeNode::new(1))))),
        tree.clone()
    );
}
