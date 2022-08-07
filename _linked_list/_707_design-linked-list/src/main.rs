/*
 * @lc app=leetcode id=707 lang=rust
 *
 * [707] Design Linked List
 */

// @lc code=start
use std::{cell::RefCell, rc::Rc};
#[derive(Debug, Clone)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
struct MyLinkedList {
    head: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { head: None }
    }

    fn get(&self, index: i32) -> i32 {
        let mut cur: &Box<Node> = match self.head {
            Some(ref head) => head,
            None => return -1,
        };

        for _ in 0..index {
            if let Some(ref next) = cur.next {
                cur = next;
            } else {
                return -1;
            };
        }
        cur.val
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }))
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut cur: &mut Box<Node> = match self.head {
            Some(ref mut head) => head,
            None => {
                self.head = Some(Box::new(Node { val, next: None }));
                return;
            }
        };

        while let Some(ref mut next) = cur.next {
            cur = next;
        }

        cur.next = Some(Box::new(Node { val, next: None }));
        return;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.head = Some(Box::new(Node {
                val,
                next: self.head.take(),
            }));
            return;
        }

        let mut cur: &mut Box<Node> = match self.head {
            Some(ref mut head) => head,
            None => return,
        };

        for _ in 0..(index - 1) {
            if let Some(ref mut next) = cur.next {
                cur = next;
            } else {
                return;
            };
        }
        cur.next = Some(Box::new(Node {
            val,
            next: cur.next.take(),
        }));
    }

    fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
            self.head = self.head.take().and_then(|head| head.next);
            return;
        }
        let mut cur: &mut Box<Node> = match self.head {
            Some(ref mut head) => head,
            None => return,
        };

        for _ in 0..(index - 1) {
            if let Some(ref mut next) = cur.next {
                cur = next;
            } else {
                return;
            };
        }
        cur.next = cur.next.take().and_then(|next| next.next);
    }
}

struct DoublyNode {
    val: i32,
    next: Option<Rc<RefCell<DoublyNode>>>,
    pre: Option<Rc<RefCell<DoublyNode>>>,
}

impl DoublyNode {
    fn new(val: i32) -> Self {
        DoublyNode {
            val,
            next: None,
            pre: None,
        }
    }
}

struct MyDoublyLinkedList {
    head: Option<Rc<RefCell<DoublyNode>>>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyDoublyLinkedList {
    fn new() -> Self {
        MyDoublyLinkedList { head: None }
    }

    fn get(&self, index: i32) -> i32 {
        let mut cur = match self.head {
            Some(ref head) => head,
            None => return -1,
        };

        for _ in 0..index {
            if let Some(ref next) = cur.borrow().next {
                cur = next;
            } else {
                return -1;
            };
        }
        cur.borrow().val
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Rc::new(RefCell::new( DoublyNode {
            val,
            next: self.head.take(),
        }))
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut cur: &mut Box<Node> = match self.head {
            Some(ref mut head) => head,
            None => {
                self.head = Some(Box::new(Node { val, next: None }));
                return;
            }
        };

        while let Some(ref mut next) = cur.next {
            cur = next;
        }

        cur.next = Some(Box::new(Node { val, next: None }));
        return;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.head = Some(Box::new(Node {
                val,
                next: self.head.take(),
            }));
            return;
        }

        let mut cur: &mut Box<Node> = match self.head {
            Some(ref mut head) => head,
            None => return,
        };

        for _ in 0..(index - 1) {
            if let Some(ref mut next) = cur.next {
                cur = next;
            } else {
                return;
            };
        }
        cur.next = Some(Box::new(Node {
            val,
            next: cur.next.take(),
        }));
    }

    fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
            self.head = self.head.take().and_then(|head| head.next);
            return;
        }
        let mut cur: &mut Box<Node> = match self.head {
            Some(ref mut head) => head,
            None => return,
        };

        for _ in 0..(index - 1) {
            if let Some(ref mut next) = cur.next {
                cur = next;
            } else {
                return;
            };
        }
        cur.next = cur.next.take().and_then(|next| next.next);
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
// @lc code=end

fn main() {
    let mut my_linked_list = MyLinkedList::new();
    my_linked_list.add_at_head(1);
    println!("{:?}", my_linked_list);
    my_linked_list.add_at_tail(3);
    println!("{:?}", my_linked_list);
    my_linked_list.add_at_index(1, 2); // linked list becomes 1->2->3
    println!("{:?}", my_linked_list);
    my_linked_list.get(1); // return 2
    println!("{:?}", my_linked_list);
    my_linked_list.delete_at_index(1); // now the linked list is 1->3
    println!("{:?}", my_linked_list);
    my_linked_list.get(1);
    println!("{:?}", my_linked_list);
}
