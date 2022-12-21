/*
 * @lc app=leetcode id=919 lang=rust
 *
 * [919] Complete Binary Tree Inserter
 *
 * https://leetcode.com/problems/complete-binary-tree-inserter/description/
 *
 * algorithms
 * Medium (64.45%)
 * Likes:    906
 * Dislikes: 86
 * Total Accepted:    43.4K
 * Total Submissions: 66.8K
 * Testcase Example:  '["CBTInserter","insert","insert","get_root"]\n[[[1,2]],[3],[4],[]]'
 *
 * A complete binary tree is a binary tree in which every level, except
 * possibly the last, is completely filled, and all nodes are as far left as
 * possible.
 *
 * Design an algorithm to insert a new node to a complete binary tree keeping
 * it complete after the insertion.
 *
 * Implement the CBTInserter class:
 *
 *
 * CBTInserter(TreeNode root) Initializes the data structure with the root of
 * the complete binary tree.
 * int insert(int v) Inserts a TreeNode into the tree with value Node.val ==
 * val so that the tree remains complete, and returns the value of the parent
 * of the inserted TreeNode.
 * TreeNode get_root() Returns the root node of the tree.
 *
 *
 *
 * Example 1:
 *
 *
 * Input
 * ["CBTInserter", "insert", "insert", "get_root"]
 * [[[1, 2]], [3], [4], []]
 * Output
 * [null, 1, 2, [1, 2, 3, 4]]
 *
 * Explanation
 * CBTInserter cBTInserter = new CBTInserter([1, 2]);
 * cBTInserter.insert(3);  // return 1
 * cBTInserter.insert(4);  // return 2
 * cBTInserter.get_root(); // return [1, 2, 3, 4]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree will be in the range [1, 1000].
 * 0 <= Node.val <= 5000
 * root is a complete binary tree.
 * 0 <= val <= 5000
 * At most 10^4 calls will be made to insert and get_root.
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

// @lc code=start
use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

struct CBTInserter {
    queue: VecDeque<Rc<RefCell<TreeNode>>>,
    root: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut queue = VecDeque::new();

        queue.push_back(root.as_ref().unwrap().clone());

        while let Some(ref node) = queue.front() {
            if node.borrow().left.is_none() || node.borrow().right.is_none() {
                break;
            }
            let node_left_child = node.borrow().left.as_ref().unwrap().clone();
            let node_right_child = node.borrow().right.as_ref().unwrap().clone();
            queue.push_back(node_left_child);
            queue.push_back(node_right_child);
            queue.pop_front();
        }

        Self { root, queue }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        let parent = self.queue.front().unwrap().clone();

        if parent.borrow().left.is_none() {
            parent.borrow_mut().left = Some(node);
            parent.borrow().val
        } else {
            parent.borrow_mut().right = Some(node.clone());
            self.queue.pop_front();
            let parent_left_child = parent.borrow().left.as_ref().unwrap().clone();
            let parent_right_child = parent.borrow().right.as_ref().unwrap().clone();
            self.queue.push_back(parent_left_child);
            self.queue.push_back(parent_right_child);
            parent.borrow().val
        }
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

/**
 * Your CBTInserter object will be instantiated and called as such:
 * let obj = CBTInserter::new(root);
 * let ret_1: i32 = obj.insert(val);
 * let ret_2: Option<Rc<RefCell<TreeNode>>> = obj.get_root();
 */
// @lc code=end

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let child = Rc::new(RefCell::new(TreeNode::new(2)));

    root.borrow_mut().left = Some(child);

    let mut cbt_inserter = CBTInserter::new(Some(root.clone()));
    assert_eq!(cbt_inserter.insert(3), 1);
    assert_eq!(cbt_inserter.insert(4), 2);
    assert_eq!(cbt_inserter.get_root(), Some(root));
}
