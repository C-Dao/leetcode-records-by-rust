/*
 * @lc app=leetcode id=430 lang=rust
 *
 * [430] Flatten a multilevel doubly linked list
 */

use std::{cell::RefCell, rc::Rc, rc::Weak};

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

fn back_track(head: Option<Rc<RefCell<DoublyNode>>>) -> (Option<Rc<RefCell<DoublyNode>>>,Option<Rc<RefCell<DoublyNode>>>) {
    let mut dummy_cur = Rc::new(RefCell::new(DoublyNode::new(-1)));
    dummy_cur.borrow_mut().next = head.clone();

    let mut tail = head.clone();

    while dummy_cur.borrow_mut().next.is_some(){
        let next_node = dummy_cur.borrow_mut().next.take();
        let child_node = dummy_cur.borrow_mut().child.take();
        
        if child_node.is_some(){  
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
        
        }else{
            dummy_cur = next_node.clone().unwrap();
        }

        tail = Some(dummy_cur.clone());
    }

     (head, tail)
 }
}

// @lc code=end


fn main() {



    println!("Hello, world!");
}
