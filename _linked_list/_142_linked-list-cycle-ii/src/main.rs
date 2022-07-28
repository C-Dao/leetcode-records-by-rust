/*
 * @lc app=leetcode id=142 lang=rust
 *
 * [141] Linked List Cycle II
 */

// @lc code=start

use std::{cell::RefCell, rc::Rc};
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
        let mut fast = match slow.borrow().next {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };

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
            println!("{:?}{:?}", fast.as_ptr(), slow.as_ptr());

            if fast.as_ptr() == slow.as_ptr() {
                let mut ptr = match head {
                    Some(ref node) => Rc::clone(node),
                    None => return None,
                };
                while ptr.as_ptr() != slow.as_ptr() {
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
                println!("{:?}{:?}", fast, slow);

                return Some(ptr.clone());
            }
        }
        None
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
    println!("{:?}", Solution::detect_cycle(Some(a.clone())));
    assert_eq!(Solution::detect_cycle(Some(a.clone())), Some(c.clone()));

    let a = Rc::new(RefCell::new(ListNode::new(1)));
    let b = Rc::new(RefCell::new(ListNode::new(2)));
    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(a.clone());
    assert_eq!(Solution::detect_cycle(Some(a.clone())), Some(a.clone()));

    let a = Rc::new(RefCell::new(ListNode::new(1)));
    assert_eq!(Solution::detect_cycle(Some(a.clone())), None);
}
