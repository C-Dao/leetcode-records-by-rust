/*
 * @lc app=leetcode id=138 lang=rust
 *
 * [138] Copy List with Random Pointer
 *
 * https://leetcode.com/problems/copy-list-with-random-pointer/description/
 *
 * algorithms
 * Medium (49.08%)
 * Likes:    9525
 * Dislikes: 1046
 * Total Accepted:    846.7K
 * Total Submissions: 1.7M
 * Testcase Example:  '[[7,null],[13,0],[11,4],[10,2],[1,0]]'
 *
 * A linked list of length n is given such that each node contains an
 * additional random pointer, which could point to any node in the list, or
 * null.
 * 
 * Construct a deep copy of the list. The deep copy should consist of exactly n
 * brand new nodes, where each new node has its value set to the value of its
 * corresponding original node. Both the next and random pointer of the new
 * nodes should point to new nodes in the copied list such that the pointers in
 * the original list and copied list represent the same list state. None of the
 * pointers in the new list should point to nodes in the original list.
 * 
 * For example, if there are two nodes X and Y in the original list, where
 * X.random --> Y, then for the corresponding two nodes x and y in the copied
 * list, x.random --> y.
 * 
 * Return the head of the copied linked list.
 * 
 * The linked list is represented in the input/output as a list of n nodes.
 * Each node is represented as a pair of [val, random_index] where:
 * 
 * 
 * val: an integer representing Node.val
 * random_index: the index of the node (range from 0 to n-1) that the random
 * pointer points to, or null if it does not point to any node.
 * 
 * 
 * Your code will only be given the head of the original linked list.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: head = [[7,null],[13,0],[11,4],[10,2],[1,0]]
 * Output: [[7,null],[13,0],[11,4],[10,2],[1,0]]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: head = [[1,1],[2,1]]
 * Output: [[1,1],[2,1]]
 * 
 * 
 * Example 3:
 * 
 * 
 * 
 * 
 * Input: head = [[3,null],[3,0],[3,null]]
 * Output: [[3,null],[3,0],[3,null]]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 0 <= n <= 1000
 * -10^4 <= Node.val <= 10^4
 * Node.random is null or is pointing to some node in the linked list.
 * 
 * 
 */

use std::collections::HashMap;
use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc, rc::Weak};

struct Solution {}

// @lc code=start
#[derive(Debug, Clone)]
struct RandomNode {
    val: i32,
    next: Option<Rc<RefCell<RandomNode>>>,
    random: Option<Weak<RefCell<RandomNode>>>,
}

impl RandomNode {
    fn new(val: i32) -> Self {
        RandomNode {
            val,
            next: None,
            random: None,
        }
    }
}

impl Solution {
    pub fn copy_random_list(
        head: Option<Rc<RefCell<RandomNode>>>,
    ) -> Option<Rc<RefCell<RandomNode>>> {
        let mut cur = head.clone();
        let mut hash_map = HashMap::new();

        if head.is_none() {
            return None;
        }

        while cur.is_some() {
            let cur_node = cur.unwrap();
            hash_map.insert(
                cur_node.borrow().val,
                Rc::new(RefCell::new(RandomNode::new(cur_node.borrow().val))),
            );
            cur = cur_node.borrow().next.clone();
        }

        cur = head.clone();

        while cur.is_some() && cur.as_ref().unwrap().borrow().next.is_some() {
            let cur_node = cur.unwrap();
            hash_map
                .get_mut(&cur_node.borrow().val)
                .unwrap()
                .borrow_mut()
                .next = Some(
                hash_map
                    .get(&cur_node.borrow().next.as_ref().unwrap().borrow().val)
                    .unwrap()
                    .clone(),
            );
            hash_map
                .get_mut(&cur_node.borrow().val)
                .unwrap()
                .borrow_mut()
                .random = Some(Rc::downgrade(
                &hash_map
                    .get(&cur_node.borrow().next.as_ref().unwrap().borrow().val)
                    .unwrap()
                    .clone(),
            ));
            cur = cur_node.borrow().next.clone();
        }

        Some(hash_map.get(&head.unwrap().borrow().val).unwrap().clone())
    }

    pub fn copy_random_list_by_special(
        head: Option<Rc<RefCell<RandomNode>>>,
    ) -> Option<Rc<RefCell<RandomNode>>> {
        let mut cur = head.clone();

        if cur.is_none() {
            return None;
        }

        while cur.is_some() {
            let cur_node = cur.unwrap();
            let cur_next = cur_node.borrow().next.clone();
            let new_node = Rc::new(RefCell::new(RandomNode::new(cur_node.borrow().val)));

            new_node.borrow_mut().next = cur_next.clone();
            cur_node.borrow_mut().next = Some(new_node);

            cur = cur_next;
        }

        cur = head.clone();
        while cur.is_some() {
            let cur_node = cur.unwrap();
            let cur_next_node = cur_node.borrow().next.clone().unwrap();

            cur_next_node.borrow_mut().random = if cur_node.borrow().random.is_some() {
                Some(Rc::downgrade(
                    &cur_node
                        .borrow()
                        .random
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .borrow()
                        .next
                        .as_ref()
                        .unwrap(),
                ))
            } else {
                None
            };
            cur = cur_next_node.borrow().next.clone();
        }

        cur = head.clone();
        let head_new = head.as_ref().unwrap().borrow().next.clone();
        while cur.is_some() {
            let cur_node = cur.unwrap();
            let cur_next_node = cur_node.borrow().next.clone().unwrap();
            let cur_next_node_next_node = cur_next_node.borrow().next.clone();

            cur_node.borrow_mut().next = cur_next_node_next_node.clone();
            cur_next_node.borrow_mut().next = if cur_next_node_next_node.is_some() {
                cur_next_node_next_node.unwrap().borrow().next.clone()
            } else {
                None
            };
            cur = cur_node.borrow().next.clone();
        }

        head_new
    }
}

// @lc code=end

fn main() {
    let a = Rc::new(RefCell::new(RandomNode::new(7)));
    let b = Rc::new(RefCell::new(RandomNode::new(13)));
    let c = Rc::new(RefCell::new(RandomNode::new(11)));
    let d = Rc::new(RefCell::new(RandomNode::new(10)));
    let e = Rc::new(RefCell::new(RandomNode::new(1)));
    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());
    b.borrow_mut().random = Some(Rc::downgrade(&a.clone()));
    c.borrow_mut().next = Some(d.clone());
    c.borrow_mut().random = Some(Rc::downgrade(&e.clone()));
    d.borrow_mut().next = Some(e.clone());
    d.borrow_mut().random = Some(Rc::downgrade(&c.clone()));
    e.borrow_mut().random = Some(Rc::downgrade(&a.clone()));

    println!("{:?}", Solution::copy_random_list(Some(a.clone())));
    println!(
        "{:?}",
        Solution::copy_random_list_by_special(Some(a.clone()))
    );
}
