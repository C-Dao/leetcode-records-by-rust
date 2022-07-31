/*
 * @lc app=leetcode id=141 lang=rust
 *
 * [141] Linked List Cycle
 */

// @lc code=start

use std::{cell::RefCell, collections::HashSet, rc::Rc};
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution {}

// @lc code=start
impl Solution {
    pub fn get_intersection_node(
        head_1: Option<Rc<RefCell<ListNode>>>,
        head_2: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        let mut pointer_one = match head_1 {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };
        let mut pointer_two = match head_2 {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };

        while pointer_one.as_ptr() != pointer_two.as_ptr() {
            let mut pointer_one_next = pointer_one.borrow().next.clone();
            let mut pointer_two_next = pointer_two.borrow().next.clone();

            if pointer_one_next.is_none() && pointer_two_next.is_none() {
                return None;
            }

            if pointer_one_next.is_none() {
                pointer_one_next = head_2.clone();
                pointer_one = pointer_one_next.unwrap().clone();
            } else if let Some(ref node) = pointer_one_next {
                pointer_one = Rc::clone(node);
            }

            if pointer_two_next.is_none() {
                pointer_two_next = head_1.clone();
                pointer_two = pointer_two_next.unwrap().clone();
            } else if let Some(ref node) = pointer_two_next {
                pointer_two = Rc::clone(node);
            }
        }
        Some(pointer_one)
    }

    pub fn get_intersection_node_hash_map_edition(
        head_1: Option<Rc<RefCell<ListNode>>>,
        head_2: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        let mut set = HashSet::new();

        let mut ptr = match head_1 {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };

        while set.insert(ptr.as_ptr()) {
            let next = match ptr.borrow().next {
                Some(ref node) => Rc::clone(node),
                None => break,
            };
            ptr = next;
        }

        ptr = match head_2 {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };
        while !set.contains(&ptr.as_ptr()) {
            let next = match ptr.borrow().next {
                Some(ref node) => Rc::clone(node),
                None => return None,
            };
            ptr = next;
        }

        Some(ptr)
    }
}
// @lc code=end

fn main() {
    let a = Rc::new(RefCell::new(ListNode::new(4)));
    let b = Rc::new(RefCell::new(ListNode::new(1)));
    let c = Rc::new(RefCell::new(ListNode::new(8)));
    let d = Rc::new(RefCell::new(ListNode::new(4)));
    let e = Rc::new(RefCell::new(ListNode::new(5)));
    let f = Rc::new(RefCell::new(ListNode::new(5)));
    let g = Rc::new(RefCell::new(ListNode::new(6)));
    let h = Rc::new(RefCell::new(ListNode::new(1)));

    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());
    c.borrow_mut().next = Some(d.clone());
    d.borrow_mut().next = Some(e.clone());

    f.borrow_mut().next = Some(g.clone());
    g.borrow_mut().next = Some(h.clone());
    h.borrow_mut().next = Some(c.clone());

    assert_eq!(
        Solution::get_intersection_node(Some(a.clone()), Some(f.clone()))
            .unwrap()
            .as_ptr(),
        c.clone().as_ptr()
    );
    assert_eq!(
        Solution::get_intersection_node_hash_map_edition(Some(a.clone()), Some(f.clone()))
            .unwrap()
            .as_ptr(),
        c.clone().as_ptr()
    );

    let a = Rc::new(RefCell::new(ListNode::new(2)));
    let b = Rc::new(RefCell::new(ListNode::new(6)));
    let c = Rc::new(RefCell::new(ListNode::new(4)));
    let d = Rc::new(RefCell::new(ListNode::new(1)));
    let e = Rc::new(RefCell::new(ListNode::new(5)));

    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());

    d.borrow_mut().next = Some(e.clone());

    assert_eq!(
        Solution::get_intersection_node(Some(a.clone()), Some(d.clone())),
        None
    );
    assert_eq!(
        Solution::get_intersection_node_hash_map_edition(Some(a.clone()), Some(d.clone())),
        None
    );

    let a = Rc::new(RefCell::new(ListNode::new(1)));
    let b = Rc::new(RefCell::new(ListNode::new(9)));
    let c = Rc::new(RefCell::new(ListNode::new(1)));
    let d = Rc::new(RefCell::new(ListNode::new(2)));
    let e = Rc::new(RefCell::new(ListNode::new(4)));
    let f = Rc::new(RefCell::new(ListNode::new(3)));

    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());
    c.borrow_mut().next = Some(d.clone());
    d.borrow_mut().next = Some(e.clone());

    f.borrow_mut().next = Some(d.clone());

    assert_eq!(
        Solution::get_intersection_node(Some(a.clone()), Some(f.clone()))
            .unwrap()
            .as_ptr(),
        d.clone().as_ptr()
    );
    assert_eq!(
        Solution::get_intersection_node_hash_map_edition(Some(a.clone()), Some(f.clone()))
            .unwrap()
            .as_ptr(),
        d.clone().as_ptr()
    );
}
