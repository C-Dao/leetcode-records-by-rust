/*
 * @lc app=leetcode id=897 lang=rust
 *
 * [897] Increasing Order Search Tree
 *
 * https://leetcode.com/problems/increasing-order-search-tree/description/
 *
 * algorithms
 * Easy (78.36%)
 * Likes:    3697
 * Dislikes: 640
 * Total Accepted:    238.1K
 * Total Submissions: 303.4K
 * Testcase Example:  '[5,3,6,2,4,null,8,1,null,null,null,7,9]'
 *
 * Given the root of a binary search tree, rearrange the tree in in-order so
 * that the leftmost node in the tree is now the root of the tree, and every
 * node has no left child and only one right child.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [5,3,6,2,4,null,8,1,null,null,null,7,9]
 * Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]
 *
 *
 * Example 2:
 *
 *
 * Input: root = [5,1,7]
 * Output: [1,null,5,null,7]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the given tree will be in the range [1, 100].
 * 0 <= Node.val <= 1000
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
    /** iteraction, mid-order traversal */
    pub fn increasing_bst_iteraction(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        let mut cur = root.clone();
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
        let mut head = None;

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let left = cur.as_ref().unwrap().borrow().left.clone();
                stack.push(cur.clone());
                cur = left;
            }

            cur = stack.pop().unwrap().clone();
            
            if prev.is_some() {
                prev.as_mut().unwrap().borrow_mut().right = cur.clone();
            } else {
                head = cur.clone();
            }

            prev = cur.clone();
            cur.as_mut().unwrap().borrow_mut().left = None;
            let cur_right = cur.as_mut().unwrap().borrow_mut().right.take();
            cur = cur_right;
        }

        head
    }

    /** recursion, mid-order traversal */
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs_recursion(root, None)
    }

    pub fn dfs_recursion(
        mut node: Option<Rc<RefCell<TreeNode>>>,
        tail: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if node.is_none() {
            return tail;
        }

        let mut node_ref = node.as_mut().unwrap().borrow_mut();

        node_ref.right = Self::dfs_recursion(node_ref.right.take(), tail);

        let left = node_ref.left.take();

        drop(node_ref);

        Self::dfs_recursion(left, node)
    }
}
// @lc code=end

fn main() {
    let mut root_0 = TreeNode::new(5);
    let node_1 = TreeNode::new(1);
    let node_2 = TreeNode::new(7);

    root_0.right = Some(Rc::new(RefCell::new(node_2)));
    root_0.left = Some(Rc::new(RefCell::new(node_1)));

    let root_0_0 = Some(Rc::new(RefCell::new(root_0)));

    let mut root_1 = TreeNode::new(5);
    let node_1 = TreeNode::new(1);
    let node_2 = TreeNode::new(7);

    root_1.right = Some(Rc::new(RefCell::new(node_2)));
    root_1.left = Some(Rc::new(RefCell::new(node_1)));

    let root_0_1 = Some(Rc::new(RefCell::new(root_1)));

    assert_eq!(
        Solution::increasing_bst(root_0_0.clone()),
        Solution::increasing_bst_iteraction(root_0_1.clone())
    );
}
