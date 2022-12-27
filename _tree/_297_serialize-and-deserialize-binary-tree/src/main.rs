/*
 * @lc app=leetcode id=297 lang=rust
 *
 * [297] Serialize and Deserialize Binary Tree
 *
 * https://leetcode.com/problems/serialize-and-deserialize-binary-tree/description/
 *
 * algorithms
 * Hard (54.25%)
 * Likes:    8200
 * Dislikes: 300
 * Total Accepted:    706.9K
 * Total Submissions: 1.3M
 * Testcase Example:  '[1,2,3,null,null,4,5]'
 *
 * Serialization is the process of converting a data structure or object into a
 * sequence of bits so that it can be stored in a file or memory buffer, or
 * transmitted across a network connection link to be reconstructed later in
 * the same or another computer environment.
 *
 * Design an algorithm to serialize and deserialize a binary tree. There is no
 * restriction on how your serialization/deserialization algorithm should work.
 * You just need to ensure that a binary tree can be serialized to a string and
 * this string can be deserialized to the original tree structure.
 *
 * Clarification: The input/output format is the same as how LeetCode
 * serializes a binary tree. You do not necessarily need to follow this format,
 * so please be creative and come up with different approaches yourself.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,3,null,null,4,5]
 * Output: [1,2,3,null,null,4,5]
 *
 *
 * Example 2:
 *
 *
 * Input: root = []
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 10^4].
 * -1000 <= Node.val <= 1000
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
use std::cell::RefCell;
use std::rc::Rc;
struct Codec {
    str_count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self { str_count: 0 }
    }

    /** recursion, pre-order traversal */
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return format!("#");
        }

        let str = root.as_ref().unwrap().borrow().val.to_string();
        let left_str = self.serialize(root.as_ref().unwrap().borrow().left.clone());
        let right_str = self.serialize(root.as_ref().unwrap().borrow().right.clone());
        return format!("{},{},{}", str, left_str, right_str);
    }

    /** iteraction, pre-order traversal  */
    fn serialize_iteraction(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut stack = vec![];
        let mut cur = root.clone();
        let mut ans = vec![];

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                ans.push(format!("{}", cur.as_ref().unwrap().borrow().val));
                stack.push(cur.clone());
                let left = cur.as_ref().unwrap().borrow().left.clone();
                cur = left;
            }

            ans.push(format!("#"));

            let parent = stack.pop().unwrap().clone();
            cur = parent.as_ref().unwrap().borrow().right.clone();
        }

        ans.push(format!("#"));

        ans.join(",")
    }

    fn deserialize(&mut self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        self.str_count = 0;
        let strs = data.split(",").collect::<Vec<&str>>();
        self.deserialize_dfs(&strs)
    }

    fn deserialize_dfs(&mut self, strs: &Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        let str = strs[self.str_count];

        self.str_count += 1;

        if str == "#" {
            return None;
        };

        let mut node = TreeNode::new(str.parse().unwrap());

        node.left = self.deserialize_dfs(strs);
        node.right = self.deserialize_dfs(strs);

        Some(Rc::new(RefCell::new(node)))
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
// @lc code=end

fn main() {
    let mut root_0 = TreeNode::new(1);
    let node_1 = TreeNode::new(2);
    let mut node_2 = TreeNode::new(3);
    let node_3 = TreeNode::new(4);
    let node_4 = TreeNode::new(5);

    node_2.left = Some(Rc::new(RefCell::new(node_3)));
    node_2.right = Some(Rc::new(RefCell::new(node_4)));

    root_0.right = Some(Rc::new(RefCell::new(node_2)));
    root_0.left = Some(Rc::new(RefCell::new(node_1)));

    let root_0_0 = Some(Rc::new(RefCell::new(root_0)));

    let mut codec = Codec::new();
    let data_0 = codec.serialize(root_0_0.clone());
    let data_1 = codec.serialize_iteraction(root_0_0.clone());

    assert_eq!(data_0.clone(), "1,2,#,#,3,4,#,#,5,#,#");
    assert_eq!(data_1.clone(), "1,2,#,#,3,4,#,#,5,#,#");
    assert_eq!(codec.deserialize(data_0), root_0_0);
    assert_eq!(codec.deserialize(data_1), root_0_0);
}
