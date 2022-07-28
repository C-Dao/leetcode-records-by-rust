/*
 * @lc app=leetcode id=142 lang=rust
 *
 * [141] Linked List Cycle II
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
    pub fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
        let mut slow = match head {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };
        let mut fast = slow.clone();

        while fast.borrow().next.is_some() {
            let next = match slow.borrow().next {
                Some(ref node) => Rc::clone(node),
                None => return None,
            };
            slow = next;

            for _ in 0..2 {
                let next = match fast.borrow().next {
                    Some(ref node) => Rc::clone(node),
                    None => return None,
                };
                fast = next;
            }

            if fast.as_ptr() == slow.as_ptr() {
                let mut ptr = match head {
                    Some(ref node) => Rc::clone(node),
                    None => return None,
                };

                loop {
                    if ptr.as_ptr() == slow.as_ptr() {
                        return Some(ptr.clone());
                    }

                    let next = match ptr.borrow().next {
                        Some(ref node) => Rc::clone(node),
                        None => return None,
                    };
                    ptr = next;

                    let next = match slow.borrow().next {
                        Some(ref node) => Rc::clone(node),
                        None => return None,
                    };
                    slow = next;
                }
            }
        }
        None
    }
    pub fn detect_cycle_hash_map_edition(
        head: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        let mut set = HashSet::new();

        let mut ptr = match head {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };

        while set.insert(ptr.as_ptr()) {
            let next = match ptr.borrow().next {
                Some(ref node) => Rc::clone(node),
                None => return None,
            };
            ptr = next;
        }

        Some(ptr.clone())
    }
}
// @lc code=end

fn main() {
    let a = Rc::new(RefCell::new(ListNode::new(3)));
    let b = Rc::new(RefCell::new(ListNode::new(2)));
    let c = Rc::new(RefCell::new(ListNode::new(0)));
    let d = Rc::new(RefCell::new(ListNode::new(4)));
    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());
    c.borrow_mut().next = Some(d.clone());
    d.borrow_mut().next = Some(c.clone());
    assert_eq!(
        Solution::detect_cycle(Some(a.clone())).unwrap().as_ptr(),
        c.clone().as_ptr()
    );
    assert_eq!(
        Solution::detect_cycle_hash_map_edition(Some(a.clone()))
            .unwrap()
            .as_ptr(),
        c.clone().as_ptr()
    );
    let a = Rc::new(RefCell::new(ListNode::new(1)));
    let b = Rc::new(RefCell::new(ListNode::new(2)));
    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(a.clone());
    assert_eq!(
        Solution::detect_cycle(Some(a.clone())).unwrap().as_ptr(),
        a.clone().as_ptr()
    );
    assert_eq!(
        Solution::detect_cycle_hash_map_edition(Some(a.clone()))
            .unwrap()
            .as_ptr(),
        a.clone().as_ptr()
    );
    let a = Rc::new(RefCell::new(ListNode::new(1)));
    assert_eq!(Solution::detect_cycle(Some(a.clone())), None);
    assert_eq!(
        Solution::detect_cycle_hash_map_edition(Some(a.clone())),
        None
    );
}
