/*
 * @lc app=leetcode id=141 lang=rust
 *
 * [141] Linked List Cycle
 */

// @lc code=start

use std::{cell::RefCell, rc::Rc};
#[allow(dead_code)]
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
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut slow = match head {
            Some(ref node) => Rc::clone(node),
            None => return false,
        };
        let mut fast = match slow.borrow().next {
            Some(ref node) => Rc::clone(node),
            None => return false,
        };

        while slow.as_ptr() != fast.as_ptr() {
            let next = match slow.borrow().next {
                Some(ref node) => Rc::clone(node),
                None => return false,
            };
            slow = next;

            for _ in 0..2 {
                let next = match fast.borrow().next {
                    Some(ref node) => Rc::clone(node),
                    None => return false,
                };
                fast = next;
            }
        }
        true
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
    d.borrow_mut().next = Some(a.clone());
    assert_eq!(Solution::has_cycle(Some(a)), true);

    let a = Rc::new(RefCell::new(ListNode::new(1)));
    let b = Rc::new(RefCell::new(ListNode::new(2)));
    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(a.clone());
    assert_eq!(Solution::has_cycle(Some(a)), true);

    let a = Rc::new(RefCell::new(ListNode::new(1)));
    assert_eq!(Solution::has_cycle(Some(a)), false);
}
