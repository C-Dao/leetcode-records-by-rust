/*
* @lc app=leetcode id=173 lang=rust
*
* [173] Binary Search Tree Iterator
*
* https://leetcode.com/problems/binary-search-tree-iterator/description/
*
* algorithms
* Medium (67.95%)
* Likes:    7139
* Dislikes: 433
* Total Accepted:    644.8K
* Total Submissions: 930.2K
* Testcase Example:  '["BSTIterator","next","next","hasNext","next","hasNext","next","hasNext","next","hasNext"]\n' +
 '[[[7,3,15,null,null,9,20]],[],[],[],[],[],[],[],[],[]]'
*
* Implement the BSTIterator class that represents an iterator over the
* in-order traversal of a binary search tree (BST):
*
*
* BSTIterator(TreeNode root) Initializes an object of the BSTIterator class.
* The root of the BST is given as part of the constructor. The pointer should
* be initialized to a non-existent number smaller than any element in the
* BST.
* boolean hasNext() Returns true if there exists a number in the traversal to
* the right of the pointer, otherwise returns false.
* int next() Moves the pointer to the right, then returns the number at the
* pointer.
*
*
* Notice that by initializing the pointer to a non-existent smallest number,
* the first call to next() will return the smallest element in the BST.
*
* You may assume that next() calls will always be valid. That is, there will
* be at least a next number in the in-order traversal when next() is
* called.
*
*
* Example 1:
*
*
* Input
* ["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next",
* "hasNext", "next", "hasNext"]
* [[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
* Output
* [null, 3, 7, true, 9, true, 15, true, 20, false]
*
* Explanation
* BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
* bSTIterator.next();    // return 3
* bSTIterator.next();    // return 7
* bSTIterator.hasNext(); // return True
* bSTIterator.next();    // return 9
* bSTIterator.hasNext(); // return True
* bSTIterator.next();    // return 15
* bSTIterator.hasNext(); // return True
* bSTIterator.next();    // return 20
* bSTIterator.hasNext(); // return False
*
*
*
* Constraints:
*
*
* The number of nodes in the tree is in the range [1, 10^5].
* 0 <= Node.val <= 10^6
* At most 10^5 calls will be made to hasNext, and next.
*
*
*
* Follow up:
*
*
* Could you implement next() and hasNext() to run in average O(1) time and use
* O(h) memory, where h is the height of the tree?
*
*
*/

use data_structure_marcos::*;
use data_structures::*;

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    cur: Option<Rc<RefCell<TreeNode>>>,
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            cur: root,
            stack: vec![],
        }
    }

    fn next(&mut self) -> i32 {
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

    fn has_next(&self) -> bool {
        self.cur.is_some() || !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// @lc code=end

fn main() {
    let root_tree = binary_tree!([7, 3, 15, null, null, 9, 20]);
    let mut bst_iterator = BSTIterator::new(root_tree);

    assert_eq!(bst_iterator.next(), 3);
    assert_eq!(bst_iterator.next(), 7);
    assert!(bst_iterator.has_next());
    assert_eq!(bst_iterator.next(), 9);
    assert!(bst_iterator.has_next());
    assert_eq!(bst_iterator.next(), 15);
    assert!(bst_iterator.has_next());
    assert_eq!(bst_iterator.next(), 20);
    assert!(!bst_iterator.has_next());
}
