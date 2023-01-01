/*
 * @lc app=leetcode id=653 lang=rust
 *
 * [653] Two Sum IV - Input is a BST
 *
 * https://leetcode.com/problems/two-sum-iv-input-is-a-bst/description/
 *
 * algorithms
 * Easy (59.07%)
 * Likes:    5444
 * Dislikes: 236
 * Total Accepted:    429K
 * Total Submissions: 703.2K
 * Testcase Example:  '[5,3,6,2,4,null,7]\n9'
 *
 * Given the root of a binary search tree and an integer k, return true if
 * there exist two elements in the BST such that their sum is equal to k, or
 * false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [5,3,6,2,4,null,7], k = 9
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: root = [5,3,6,2,4,null,7], k = 28
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [1, 10^4].
 * -10^4 <= Node.val <= 10^4
 * root is guaranteed to be a valid binary search tree.
 * -10^5 <= k <= 10^5
 *
 *
 */

use data_structure_marcos::*;
use data_structures::*;

struct Solution {}

// @lc code=start
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

struct BSTIteratorReversed {
    cur: Option<Rc<RefCell<TreeNode>>>,
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl BSTIteratorReversed {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            cur: root,
            stack: Vec::new(),
        }
    }

    pub fn has_prev(&self) -> bool {
        self.cur.is_some() || !self.stack.is_empty()
    }

    pub fn prev(&mut self) -> i32 {
        while self.cur.is_some() {
            let right = self.cur.as_ref().unwrap().borrow().right.clone();
            self.stack.push(self.cur.clone());
            self.cur = right;
        }

        self.cur = self.stack.pop().unwrap();

        let val = self.cur.as_ref().unwrap().borrow().val;

        let left = self.cur.as_ref().unwrap().borrow().left.clone();
        self.cur = left;

        val
    }
}

struct BSTIterator {
    cur: Option<Rc<RefCell<TreeNode>>>,
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            cur: root,
            stack: Vec::new(),
        }
    }

    pub fn has_next(&self) -> bool {
        self.cur.is_some() || !self.stack.is_empty()
    }

    pub fn next(&mut self) -> i32 {
        while self.cur.is_some() {
            let left = self.cur.as_ref().unwrap().borrow().left.clone();
            self.stack.push(self.cur.clone());
            self.cur = left;
        }

        self.cur = self.stack.pop().unwrap();

        let val = self.cur.as_ref().unwrap().borrow().val;

        let right = self.cur.as_ref().unwrap().borrow().right.clone();
        self.cur = right;

        val
    }
}

impl Solution {
    /** inorder traversal, iteraction */
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut num_set = HashSet::new();
        let mut cur = root;
        let mut stack = vec![];

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let left = cur.as_ref().unwrap().borrow().left.clone();
                stack.push(cur);
                cur = left;
            }

            cur = stack.pop().unwrap();

            if num_set.contains(&(k - cur.as_ref().unwrap().borrow().val)) {
                return true;
            }

            num_set.insert(cur.as_ref().unwrap().borrow().val);

            let right = cur.as_ref().unwrap().borrow().right.clone();
            cur = right;
        }

        false
    }

    /** inorder traversal, recursion */
    pub fn find_target_recursion(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut ans = false;
        let mut num_set = HashSet::new();

        Self::dfs_recursion(root, k, &mut ans, &mut num_set);

        ans
    }

    pub fn dfs_recursion(
        root: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
        flag: &mut bool,
        num_set: &mut HashSet<i32>,
    ) {
        if root.is_none() {
            return;
        }

        Self::dfs_recursion(
            root.as_ref().unwrap().borrow().left.clone(),
            k,
            flag,
            num_set,
        );

        if num_set.contains(&(k - root.as_ref().unwrap().borrow().val)) {
            *flag = true;
            return;
        }

        num_set.insert(root.as_ref().unwrap().borrow().val);

        Self::dfs_recursion(
            root.as_ref().unwrap().borrow().right.clone(),
            k,
            flag,
            num_set,
        );
    }

    pub fn find_target_bst_iterator(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if root.is_none() {
            return false;
        };

        let mut iter_next = BSTIterator::new(root.clone());
        let mut iter_prev = BSTIteratorReversed::new(root.clone());

        let mut next = iter_next.next();
        let mut prev = iter_prev.prev();

        while next != prev {
            if next + prev == k {
                return true;
            }

            if next + prev < k {
                next = iter_next.next();
            } else {
                prev = iter_prev.prev();
            }
        }

        false
    }
}
// @lc code=end

fn main() {
    let tree = binary_tree!([5, 3, 6, 2, 4, null, 7]);
    let k = 9;

    assert_eq!(Solution::find_target_recursion(tree.clone(), k), true);
    assert_eq!(Solution::find_target(tree.clone(), k), true);
    assert_eq!(Solution::find_target_bst_iterator(tree, k), true);
}
