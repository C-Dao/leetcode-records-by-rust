/*
 * @lc app=leetcode id=430 lang=rust
 *
 * [430] Flatten a Multilevel Doubly Linked List
 *
 * https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/description/
 *
 * algorithms
 * Medium (59.07%)
 * Likes:    3938
 * Dislikes: 274
 * Total Accepted:    249.1K
 * Total Submissions: 421.2K
 * Testcase Example:  '[1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]'
 *
 * You are given a doubly linked list, which contains nodes that have a next
 * pointer, a previous pointer, and an additional child pointer. This child
 * pointer may or may not point to a separate doubly linked list, also
 * containing these special nodes. These child lists may have one or more
 * children of their own, and so on, to produce a multilevel data structure as
 * shown in the example below.
 * 
 * Given the head of the first level of the list, flatten the list so that all
 * the nodes appear in a single-level, doubly linked list. Let curr be a node
 * with a child list. The nodes in the child list should appear after curr and
 * before curr.next in the flattened list.
 * 
 * Return the head of the flattened list. The nodes in the list must have all
 * of their child pointers set to null.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: head = [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
 * Output: [1,2,3,7,8,11,12,9,10,4,5,6]
 * Explanation: The multilevel linked list in the input is shown.
 * After flattening the multilevel linked list it becomes:
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: head = [1,2,null,3]
 * Output: [1,3,2]
 * Explanation: The multilevel linked list in the input is shown.
 * After flattening the multilevel linked list it becomes:
 * 
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: head = []
 * Output: []
 * Explanation: There could be empty list in the input.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of Nodes will not exceed 1000.
 * 1 <= Node.val <= 10^5
 * 
 * 
 * 
 * How the multilevel linked list is represented in test cases:
 * 
 * We use the multilevel linked list from Example 1 above:
 * 
 * 
 * ⁠1---2---3---4---5---6--NULL
 * ⁠        |
 * ⁠        7---8---9---10--NULL
 * ⁠            |
 * ⁠            11--12--NULL
 * 
 * The serialization of each level is as follows:
 * 
 * 
 * [1,2,3,4,5,6,null]
 * [7,8,9,10,null]
 * [11,12,null]
 * 
 * 
 * To serialize all levels together, we will add nulls in each level to signify
 * no node connects to the upper node of the previous level. The serialization
 * becomes:
 * 
 * 
 * [1,    2,    3, 4, 5, 6, null]
 * ⁠            |
 * [null, null, 7,    8, 9, 10, null]
 * ⁠                  |
 * [            null, 11, 12, null]
 * 
 * 
 * Merging the serialization of each level and removing trailing nulls we
 * obtain:
 * 
 * 
 * [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
 * 
 * 
 */

use std::fmt::{Debug, Error, Formatter};
use std::{cell::Ref, cell::RefCell, rc::Rc, rc::Weak};

struct Solution {}
// @lc code=start
#[derive(Debug, Clone)]
struct DoublyNode {
    val: i32,
    next: Option<Rc<RefCell<DoublyNode>>>,
    prev: Option<Weak<RefCell<DoublyNode>>>,
    child: Option<Rc<RefCell<DoublyNode>>>,
}

impl DoublyNode {
    fn new(val: i32) -> Self {
        DoublyNode {
            val,
            next: None,
            prev: None,
            child: None,
        }
    }
}

impl Solution {
    pub fn flatten(head: Option<Rc<RefCell<DoublyNode>>>) -> Option<Rc<RefCell<DoublyNode>>> {
        Solution::back_track(head.clone());
        return head;
    }

    fn back_track(
        head: Option<Rc<RefCell<DoublyNode>>>,
    ) -> (
        Option<Rc<RefCell<DoublyNode>>>,
        Option<Rc<RefCell<DoublyNode>>>,
    ) {
        let mut dummy_cur = Rc::new(RefCell::new(DoublyNode {
            val: -1,
            child: None,
            next: head.clone(),
            prev: None,
        }));

        let mut tail = head.clone();

        while dummy_cur.borrow_mut().next.is_some() {
            let next_node = dummy_cur.borrow_mut().next.take();
            let child_node = dummy_cur.borrow_mut().child.take();

            if child_node.is_some() {
                let (child_head, child_tail) = Solution::back_track(child_node);

                let child_head_cur = match child_head {
                    Some(ref node) => Rc::clone(node),
                    None => return (None, None),
                };

                let child_tail_cur = match child_tail {
                    Some(ref node) => Rc::clone(node),
                    None => return (None, None),
                };

                child_head_cur.borrow_mut().prev = Some(Rc::downgrade(&dummy_cur));
                child_tail_cur.borrow_mut().next = next_node;

                dummy_cur.borrow_mut().next = child_head;
                dummy_cur.borrow_mut().child = None;
                dummy_cur = child_tail_cur.borrow().next.clone().unwrap();
            } else {
                dummy_cur.borrow_mut().next = next_node.clone();
                dummy_cur = next_node.clone().unwrap();
            }

            tail = Some(dummy_cur.clone());
        }

        (head, tail)
    }
}

// @lc code=end

fn main() {
    let a = Rc::new(RefCell::new(DoublyNode::new(3)));
    let b = Rc::new(RefCell::new(DoublyNode::new(2)));
    let c = Rc::new(RefCell::new(DoublyNode::new(0)));
    let d = Rc::new(RefCell::new(DoublyNode::new(4)));
    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().prev = Some(Rc::downgrade(&a));
    b.borrow_mut().next = Some(c.clone());
    c.borrow_mut().prev = Some(Rc::downgrade(&b));
    c.borrow_mut().next = Some(d.clone());
    d.borrow_mut().prev = Some(Rc::downgrade(&c));

    let e = Rc::new(RefCell::new(DoublyNode::new(3)));
    let f = Rc::new(RefCell::new(DoublyNode::new(2)));
    let g = Rc::new(RefCell::new(DoublyNode::new(0)));

    e.borrow_mut().next = Some(f.clone());
    f.borrow_mut().prev = Some(Rc::downgrade(&e));
    f.borrow_mut().next = Some(g.clone());
    g.borrow_mut().prev = Some(Rc::downgrade(&f));

    b.borrow_mut().child = Some(e.clone());

    println!("{:?}", Solution::flatten(Some(a)));
}
