/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
 *
 * https://leetcode.com/problems/same-tree/description/
 *
 * algorithms
 * Easy (55.85%)
 * Likes:    8439
 * Dislikes: 168
 * Total Accepted:    1.4M
 * Total Submissions: 2.4M
 * Testcase Example:  '[1,2,3]\n[1,2,3]'
 *
 * Given the roots of two binary trees p and q, write a function to check if
 * they are the same or not.
 *
 * Two binary trees are considered the same if they are structurally identical,
 * and the nodes have the same value.
 *
 *
 * Example 1:
 *
 *
 * Input: p = [1,2,3], q = [1,2,3]
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: p = [1,2], q = [1,null,2]
 * Output: false
 *
 *
 * Example 3:
 *
 *
 * Input: p = [1,2,1], q = [1,1,2]
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in both trees is in the range [0, 100].
 * -10^4 <= Node.val <= 10^4
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
    /** pre-order traversal, recursion */
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        };

        if p.is_none() || q.is_none() {
            return false;
        };

        if p.as_ref().unwrap().borrow().val != q.as_ref().unwrap().borrow().val {
            return false;
        };

        return Self::is_same_tree(
            p.as_ref().unwrap().borrow().left.clone(),
            q.as_ref().unwrap().borrow().left.clone(),
        ) && Self::is_same_tree(
            p.as_ref().unwrap().borrow().right.clone(),
            q.as_ref().unwrap().borrow().right.clone(),
        );
    }

    /** bfs, breadth first search */
    pub fn is_same_tree_breadth_first_search(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack = vec![(p, q)];
        while !stack.is_empty() {
            let (p, q) = stack.pop().unwrap();
            if p.is_some()
                && q.is_some()
                && p.as_ref().unwrap().borrow().val == q.as_ref().unwrap().borrow().val
            {
                stack.push((
                    p.as_ref().unwrap().borrow().left.clone(),
                    q.as_ref().unwrap().borrow().left.clone(),
                ));
                stack.push((
                    p.as_ref().unwrap().borrow().right.clone(),
                    q.as_ref().unwrap().borrow().right.clone(),
                ));
            } else if p.is_some() || q.is_some() {
                return false;
            }
        }
        return true;
    }
}
// @lc code=end

fn main() {
    assert!(Solution::is_same_tree(
        binary_tree!([1, 2, 3]),
        binary_tree!([1, 2, 3])
    ));
    assert!(Solution::is_same_tree_breadth_first_search(
        binary_tree!([1, 2, 3]),
        binary_tree!([1, 2, 3])
    ));
}
