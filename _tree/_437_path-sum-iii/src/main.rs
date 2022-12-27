/*
 * @lc app=leetcode id=437 lang=rust
 *
 * [437] Path Sum III
 *
 * https://leetcode.com/problems/path-sum-iii/description/
 *
 * algorithms
 * Medium (49.95%)
 * Likes:    9040
 * Dislikes: 433
 * Total Accepted:    428.5K
 * Total Submissions: 884K
 * Testcase Example:  '[10,5,-3,3,2,null,11,3,-2,null,1]\n8'
 *
 * Given the root of a binary tree and an integer targetSum, return the number
 * of paths where the sum of the values along the path equals targetSum.
 *
 * The path does not need to start or end at the root or a leaf, but it must go
 * downwards (i.e., traveling only from parent nodes to child nodes).
 *
 *
 * Example 1:
 *
 *
 * Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
 * Output: 3
 * Explanation: The paths that sum to 8 are shown.
 *
 *
 * Example 2:
 *
 *
 * Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
 * Output: 3
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 1000].
 * -10^9 <= Node.val <= 10^9
 * -1000 <= targetSum <= 1000
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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);
        Self::dfs_recursion(&root, &mut map, target_sum, 0)
    }

    pub fn path_sum_iteraction(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Self::dfs_iteraction(root, target_sum)
    }

    /** pre-order traversal */
    pub fn dfs_recursion(
        root: &Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<i64, i32>,
        sum: i32,
        mut path: i64,
    ) -> i32 {
        if root.is_none() {
            return 0;
        };

        path += root.as_ref().unwrap().borrow().val as i64;

        let mut count = *map.get(&(path - sum as i64)).unwrap_or(&0);
      
        map.insert(path, *map.get(&path).unwrap_or(&0) + 1);

        count += Self::dfs_recursion(&root.as_ref().unwrap().borrow().left, map, sum, path);
        count += Self::dfs_recursion(&root.as_ref().unwrap().borrow().right, map, sum, path);

        map.insert(path, map.get(&path).unwrap_or(&0) - 1);

        count
    }

    /** pre-order traversal */
    pub fn dfs_iteraction(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut cur = root;
        let mut stack = vec![];
        let mut map = HashMap::new();
        let mut path: i64 = 0;
        let mut count = 0;
        map.insert(0, 1);

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                path += cur.as_ref().unwrap().borrow().val as i64;
                count += *map.get(&(path - sum as i64)).unwrap_or(&0);

                map.insert(path, map.get(&path).unwrap_or(&0) + 1);

                let left = cur.as_ref().unwrap().borrow().left.clone();
                stack.push((cur, path, map.clone()));
                cur = left;
            }

            let (parent, parent_path, parent_map) = stack.pop().unwrap();
            cur = parent.as_ref().unwrap().borrow().right.clone();
            path = parent_path;
            map = parent_map;
        }

        count
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

    assert_eq!(Solution::path_sum(root_0_0.clone(), 3), 2);
    assert_eq!(Solution::path_sum_iteraction(root_0_0.clone(), 3), 2);
}
