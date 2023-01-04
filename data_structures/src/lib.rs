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

#[derive(Default)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut node = self;

        for u in word.as_bytes() {
            let index = (*u - b'a') as usize;
            node = node.children[index].get_or_insert_with(|| Box::new(Trie::new()));
        }

        node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = self;

        for u in word.as_bytes() {
            let index = (*u - b'a') as usize;

            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap();
        }

        return node.is_word;
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;

        for u in prefix.as_bytes() {
            let index = (*u - b'a') as usize;

            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap();
        }

        true
    }
}
