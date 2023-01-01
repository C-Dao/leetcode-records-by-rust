use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl ListNode {
   pub fn new(val: i32) -> Self {
        Self {
            val: val,
            next: None,
        }
    }
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val: val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}
