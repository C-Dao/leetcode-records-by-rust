/*
 * @lc app=leetcode id=814 lang=rust
 *
 * [814] Binary Tree Pruning
 *
 * https://leetcode.com/problems/binary-tree-pruning/description/
 *
 * algorithms
 * Medium (70.85%)
 * Likes:    4063
 * Dislikes: 106
 * Total Accepted:    218.7K
 * Total Submissions: 301.6K
 * Testcase Example:  '[1,null,0,0,1]'
 *
 * Given the root of a binary tree, return the same tree where every subtree
 * (of the given tree) not containing a 1 has been removed.
 *
 * A subtree of a node node is node plus every node that is a descendant of
 * node.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,null,0,0,1]
 * Output: [1,null,0,null,1]
 * Explanation:
 * Only the red nodes satisfy the property "every subtree not containing a 1".
 * The diagram on the right represents the answer.
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,0,1,0,0,0,1]
 * Output: [1,null,1,null,1]
 *
 *
 * Example 3:
 *
 *
 * Input: root = [1,1,0,1,1,0,1,0]
 * Output: [1,1,0,1,1,null,1]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [1, 200].
 * Node.val is either 0 or 1.
 *
 *
 */
struct Solution {}

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

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /** recursion , pass-order traversal */
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let left_child = Self::prune_tree(root.as_ref().unwrap().borrow_mut().left.take());
        let right_child = Self::prune_tree(root.as_ref().unwrap().borrow_mut().right.take());

        root.as_ref().unwrap().borrow_mut().left = left_child;
        root.as_ref().unwrap().borrow_mut().right = right_child;

        if root.as_ref().unwrap().borrow().left.is_none()
            && root.as_ref().unwrap().borrow().right.is_none()
            && root.as_ref().unwrap().borrow().val == 0
        {
            return None;
        };

        return root;
    }

    /** iteraction, pass-order traversal */
    pub fn prune_tree_iteraction(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        };

        let mut stack = vec![];
        let mut cur = root.clone();
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                stack.push(cur.clone());
                let left = cur.as_ref().unwrap().borrow_mut().left.clone();
                cur = left;
            }

            cur = stack.last().unwrap().clone();

            if cur.as_ref().unwrap().borrow().left.is_some()
                && cur.as_ref().unwrap().borrow().left == prev
                && prev.is_some()
                && prev.as_ref().unwrap().borrow().left.is_none()
                && prev.as_ref().unwrap().borrow().right.is_none()
                && prev.as_ref().unwrap().borrow().val == 0
            {
                cur.as_mut().unwrap().borrow_mut().left = None;
            }

            if cur.as_ref().unwrap().borrow().right.is_some()
                && cur.as_ref().unwrap().borrow().right != prev
            {
                let right = cur.as_ref().unwrap().borrow().right.clone();
                cur = right;
            } else {
                if cur.as_ref().unwrap().borrow().right.is_some()
                    && cur.as_ref().unwrap().borrow().right == prev
                    && prev.is_some()
                    && prev.as_ref().unwrap().borrow().left.is_none()
                    && prev.as_ref().unwrap().borrow().right.is_none()
                    && prev.as_ref().unwrap().borrow().val == 0
                {
                    cur.as_mut().unwrap().borrow_mut().right = None;
                }

                stack.pop();
                prev = cur;
                cur = None;
            }
        }

        if root.is_some()
            && root.as_ref().unwrap().borrow().left.is_none()
            && root.as_ref().unwrap().borrow().right.is_none()
            && root.as_ref().unwrap().borrow().val == 0
        {
            None
        } else {
            root
        }
    }
}
// @lc code=end

fn main() {
    let mut root_0 = TreeNode::new(1);
    let node_1 = TreeNode::new(0);
    let mut node_2 = TreeNode::new(1);
    let node_3 = TreeNode::new(0);
    let mut node_4 = TreeNode::new(0);
    let node_5 = TreeNode::new(0);
    let node_6 = TreeNode::new(1);

    node_4.left = Some(Rc::new(RefCell::new(node_5)));
    node_4.right = Some(Rc::new(RefCell::new(node_6)));

    node_2.left = Some(Rc::new(RefCell::new(node_3)));
    node_2.right = Some(Rc::new(RefCell::new(node_4)));

    root_0.right = Some(Rc::new(RefCell::new(node_2)));
    root_0.left = Some(Rc::new(RefCell::new(node_1)));

    let mut root_1 = TreeNode::new(1);
    let mut node_2 = TreeNode::new(1);
    let mut node_4 = TreeNode::new(0);
    let node_6 = TreeNode::new(1);

    node_4.right = Some(Rc::new(RefCell::new(node_6)));

    node_2.right = Some(Rc::new(RefCell::new(node_4)));

    root_1.right = Some(Rc::new(RefCell::new(node_2)));

    assert_eq!(
        Solution::prune_tree(Some(Rc::new(RefCell::new(root_0)))),
        Some(Rc::new(RefCell::new(root_1)))
    );

    let mut root_0 = TreeNode::new(1);
    let mut node_1 = TreeNode::new(0);
    let node_2 = TreeNode::new(0);
    let node_3 = TreeNode::new(1);

    node_1.left = Some(Rc::new(RefCell::new(node_2)));
    node_1.right = Some(Rc::new(RefCell::new(node_3)));

    root_0.right = Some(Rc::new(RefCell::new(node_1)));

    let mut root_1 = TreeNode::new(1);
    let mut node_1 = TreeNode::new(0);
    let node_2 = TreeNode::new(1);

    node_1.right = Some(Rc::new(RefCell::new(node_2)));

    root_1.right = Some(Rc::new(RefCell::new(node_1)));

    assert_eq!(
        Solution::prune_tree_iteraction(Some(Rc::new(RefCell::new(root_0)))),
        Some(Rc::new(RefCell::new(root_1)))
    );
}
