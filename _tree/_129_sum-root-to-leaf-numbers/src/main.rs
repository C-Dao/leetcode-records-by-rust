/*
 * @lc app=leetcode id=129 lang=rust
 *
 * [129] Sum Root to Leaf Numbers
 *
 * https://leetcode.com/problems/sum-root-to-leaf-numbers/description/
 *
 * algorithms
 * Medium (57.63%)
 * Likes:    5218
 * Dislikes: 95
 * Total Accepted:    522K
 * Total Submissions: 885.8K
 * Testcase Example:  '[1,2,3]'
 *
 * You are given the root of a binary tree containing digits from 0 to 9 only.
 *
 * Each root-to-leaf path in the tree represents a number.
 *
 *
 * For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
 *
 *
 * Return the total sum of all root-to-leaf numbers. Test cases are generated
 * so that the answer will fit in a 32-bit integer.
 *
 * A leaf node is a node with no children.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,3]
 * Output: 25
 * Explanation:
 * The root-to-leaf path 1->2 represents the number 12.
 * The root-to-leaf path 1->3 represents the number 13.
 * Therefore, sum = 12 + 13 = 25.
 *
 *
 * Example 2:
 *
 *
 * Input: root = [4,9,0,5,1]
 * Output: 1026
 * Explanation:
 * The root-to-leaf path 4->9->5 represents the number 495.
 * The root-to-leaf path 4->9->1 represents the number 491.
 * The root-to-leaf path 4->0 represents the number 40.
 * Therefore, sum = 495 + 491 + 40 = 1026.
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [1, 1000].
 * 0 <= Node.val <= 9
 * The depth of the tree will not exceed 10.
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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs_recursion(root, 0)
    }

    pub fn sum_numbers_iteraction(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs_iteraction(root)
    }

    /** pre-order traversal */
    pub fn dfs_recursion(root: Option<Rc<RefCell<TreeNode>>>, mut path: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        path = path * 10 + root.as_ref().unwrap().borrow().val;

        if root.as_ref().unwrap().borrow().left.is_none()
            && root.as_ref().unwrap().borrow().right.is_none()
        {
            return path;
        }

        Self::dfs_recursion(root.as_ref().unwrap().borrow().left.clone(), path)
            + Self::dfs_recursion(root.as_ref().unwrap().borrow().right.clone(), path)
    }

    /** pre-order traversal */
    pub fn dfs_iteraction(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut path = 0;
        let mut stack = vec![];
        let mut cur = root;
        let mut ans = 0;
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                path = path * 10 + cur.as_ref().unwrap().borrow().val;

                if cur.as_ref().unwrap().borrow().left.is_none()
                    && cur.as_ref().unwrap().borrow().right.is_none()
                {
                    ans += path;
                }

                let left = cur.as_ref().unwrap().borrow().left.clone();
                stack.push((cur, path));
                cur = left;
            }

            let (parent, parent_path) = stack.pop().unwrap();
            cur = parent.as_ref().unwrap().borrow().right.clone();
            path = parent_path;
        }

        ans
    }
}
// @lc code=end

fn main() {
    let mut root_0 = TreeNode::new(1);
    let node_1 = TreeNode::new(2);
    let node_2 = TreeNode::new(3);

    root_0.right = Some(Rc::new(RefCell::new(node_2)));
    root_0.left = Some(Rc::new(RefCell::new(node_1)));

    let root_0_0 = Some(Rc::new(RefCell::new(root_0)));

    assert_eq!(Solution::sum_numbers(root_0_0.clone()), 25);
    assert_eq!(Solution::sum_numbers_iteraction(root_0_0.clone()), 25);
}
