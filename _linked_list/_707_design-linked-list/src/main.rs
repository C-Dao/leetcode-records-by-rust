/*
 * @lc app=leetcode id=707 lang=rust
 *
 * [707] Design Linked List
 *
 * https://leetcode.com/problems/design-linked-list/description/
 *
 * algorithms
 * Medium (27.17%)
 * Likes:    1652
 * Dislikes: 1250
 * Total Accepted:    207.5K
 * Total Submissions: 761.8K
 * Testcase Example:  '["MyLinkedList","addAtHead","addAtTail","addAtIndex","get","deleteAtIndex","get"]\n' +
  '[[],[1],[3],[1,2],[1],[1],[1]]'
 *
 * Design your implementation of the linked list. You can choose to use a
 * singly or doubly linked list.
 * A node in a singly linked list should have two attributes: val and next. val
 * is the value of the current node, and next is a pointer/reference to the
 * next node.
 * If you want to use the doubly linked list, you will need one more attribute
 * prev to indicate the previous node in the linked list. Assume all nodes in
 * the linked list are 0-indexed.
 * 
 * Implement the MyLinkedList class:
 * 
 * 
 * MyLinkedList() Initializes the MyLinkedList object.
 * int get(int index) Get the value of the index^th node in the linked list. If
 * the index is invalid, return -1.
 * void addAtHead(int val) Add a node of value val before the first element of
 * the linked list. After the insertion, the new node will be the first node of
 * the linked list.
 * void addAtTail(int val) Append a node of value val as the last element of
 * the linked list.
 * void addAtIndex(int index, int val) Add a node of value val before the
 * index^th node in the linked list. If index equals the length of the linked
 * list, the node will be appended to the end of the linked list. If index is
 * greater than the length, the node will not be inserted.
 * void deleteAtIndex(int index) Delete the index^th node in the linked list,
 * if the index is valid.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input
 * ["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get",
 * "deleteAtIndex", "get"]
 * [[], [1], [3], [1, 2], [1], [1], [1]]
 * Output
 * [null, null, null, null, 2, null, 3]
 * 
 * Explanation
 * MyLinkedList myLinkedList = new MyLinkedList();
 * myLinkedList.addAtHead(1);
 * myLinkedList.addAtTail(3);
 * myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
 * myLinkedList.get(1);              // return 2
 * myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
 * myLinkedList.get(1);              // return 3
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 0 <= index, val <= 1000
 * Please do not use the built-in LinkedList library.
 * At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and
 * deleteAtIndex.
 * 
 * 
 */


// @lc code=start
use std::fmt::{Debug, Error, Formatter};
use std::{cell::Ref, cell::RefCell, rc::Rc, rc::Weak};
#[derive(Debug, Clone)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }
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
                self.head = Some(Box::new(Node::new(val)));
                return;
            }
        };

        while let Some(ref mut next) = cur.next {
            cur = next;
        }

        cur.next = Some(Box::new(Node::new(val)));
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
#[derive(Debug, Clone)]
struct DoublyNode {
    val: i32,
    next: Option<Rc<RefCell<DoublyNode>>>,
    prev: Option<Weak<RefCell<DoublyNode>>>,
}

impl DoublyNode {
    fn new(val: i32) -> Self {
        DoublyNode {
            val,
            next: None,
            prev: None,
        }
    }
}
struct MyDoublyLinkedList {
    head: Option<Rc<RefCell<DoublyNode>>>,
    tail: Option<Weak<RefCell<DoublyNode>>>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyDoublyLinkedList {
    fn new() -> Self {
        MyDoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn get(&self, index: i32) -> i32 {
        let mut val = -1;
        if index >= 0 {
            let mut cur = match self.head {
                Some(ref head) => Rc::clone(head),
                None => return val,
            };
            for _ in 0..index {
                let next = match cur.borrow().next {
                    Some(ref node) => Rc::clone(node),
                    None => return val,
                };
                cur = next;
            }
            val = cur.borrow().val;
        }

        if index < 0 {
            let mut cur = match self.tail {
                Some(ref tail) => tail.upgrade().unwrap(),
                None => return val,
            };
            for _ in 0..index.abs() {
                let prev = match cur.borrow().prev {
                    Some(ref node) => node.upgrade().unwrap(),
                    None => return val,
                };
                cur = prev;
            }
            val = cur.borrow().val;
        }

        val
    }

    fn add_at_head(&mut self, val: i32) {
        let next = self.head.take();

        let head = Rc::new(RefCell::new(DoublyNode::new(val)));

        if next.is_some() {
            next.as_ref().unwrap().borrow_mut().prev = Some(Rc::downgrade(&head));
            head.borrow_mut().next = next;
            self.head = Some(head);
        } else {
            self.tail = Some(Rc::downgrade(&head));
            self.head = Some(head);
        }
    }

    fn add_at_tail(&mut self, val: i32) {
        let prev = self.tail.take();

        let tail = Rc::new(RefCell::new(DoublyNode::new(val)));

        if prev.is_some() {
            prev.as_ref().unwrap().upgrade().unwrap().borrow_mut().next = Some(tail.clone());
            tail.borrow_mut().prev = prev;
            self.tail = Some(Rc::downgrade(&tail));
        } else {
            self.head = Some(tail.clone());
            self.tail = Some(Rc::downgrade(&tail));
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.add_at_head(val);
            return;
        }

        if index > 0 {
            let mut cur = match self.head {
                Some(ref head) => Rc::clone(head),
                None => return,
            };

            for _ in 0..(index - 1) {
                let next = match cur.borrow().next {
                    Some(ref node) => Rc::clone(node),
                    None => return,
                };
                cur = next;
            }
            let next = Rc::new(RefCell::new(DoublyNode::new(val)));

            next.borrow_mut().prev = Some(Rc::downgrade(&cur));
            cur.borrow_mut().next = Some(next);
            return;
        }

        if index == -1 {
            self.add_at_tail(val);
            return;
        }

        let mut cur = match self.tail {
            Some(ref tail) => tail.upgrade().unwrap(),
            None => return,
        };

        for _ in 0..(index.abs() - 1) {
            let prev = match cur.borrow().prev {
                Some(ref node) => node.upgrade().unwrap(),
                None => return,
            };
            cur = prev;
        }
        let prev = Rc::new(RefCell::new(DoublyNode::new(val)));

        prev.borrow_mut().next = Some(cur.clone());
        cur.borrow_mut().prev = Some(Rc::downgrade(&prev));
    }

    fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
            self.head = self.head.take().and_then(|head| {
                let next = match head.borrow().next {
                    Some(ref node) => Rc::clone(node),
                    None => {
                        self.tail = None;
                        return None;
                    }
                };
                next.borrow_mut().prev = None;
                head.borrow_mut().next.take()
            });
            return;
        }

        if index > 0 {
            let mut cur = match self.head {
                Some(ref head) => Rc::clone(head),
                None => return,
            };

            for _ in 0..(index - 1) {
                let next = match cur.borrow().next {
                    Some(ref node) => Rc::clone(node),
                    None => return,
                };
                cur = next;
            }
            let next = cur.borrow_mut().next.take().and_then(|deleted| {
                let next = match deleted.borrow().next {
                    Some(ref node) => Rc::clone(node),
                    None => return None,
                };
                next.borrow_mut().prev = Some(Rc::downgrade(&cur));
                deleted.borrow_mut().next.take()
            });
            cur.borrow_mut().next = next;
            return;
        }

        if index == -1 {
            self.tail = self.tail.take().and_then(|tail| {
                let prev = match tail.upgrade().unwrap().borrow().next {
                    Some(ref node) => Rc::clone(node),
                    None => {
                        self.head = None;
                        return None;
                    }
                };
                prev.borrow_mut().next = None;
                tail.upgrade().unwrap().borrow_mut().prev.take()
            });
            return;
        }

        let mut cur = match self.tail {
            Some(ref tail) => tail.upgrade().unwrap(),
            None => return,
        };

        for _ in 0..(index.abs() - 1) {
            let prev = match cur.borrow().prev {
                Some(ref node) => node.upgrade().unwrap(),
                None => return,
            };
            cur = prev;
        }
        cur.borrow_mut().prev = cur.borrow_mut().prev.take().and_then(|deleted| {
            let prev = match deleted.upgrade().unwrap().borrow().prev {
                Some(ref node) => node.upgrade().unwrap(),
                None => return None,
            };
            prev.borrow_mut().next = Some(cur.clone());
            deleted.upgrade().unwrap().borrow_mut().prev.take()
        });
    }
}

impl Debug for MyDoublyLinkedList {
    fn fmt(&self, f: &'_ mut Formatter) -> Result<(), Error> {
        let mut next_node = self.head.clone();

        while let Some(node) = next_node {
            let next = node.borrow().next.clone();

            let prev_val = node
                .borrow()
                .prev
                .clone()
                .map(|p| p.upgrade().unwrap().borrow().val);

            let next_val = next.as_ref().map(|t| Ref::map(t.borrow(), |s| &s.val));

            let cur_val = Some(node.borrow().val);

            writeln!(
                f,
                "{:?} << prev << {:?} >> next >> {:?}",
                prev_val, cur_val, next_val
            )?;

            next_node = node.borrow().next.clone();
        }

        return Ok(());
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

    let mut my_doubly_linked_list = MyDoublyLinkedList::new();
    my_doubly_linked_list.add_at_head(1);
    println!("{:?}", my_doubly_linked_list);
    my_doubly_linked_list.add_at_tail(3);
    println!("{:?}", my_doubly_linked_list);
    my_doubly_linked_list.add_at_index(1, 2); // linked list becomes 1->2->3
    println!("{:?}", my_doubly_linked_list);
    my_doubly_linked_list.get(1); // return 2
    println!("{:?}", my_doubly_linked_list);
    my_doubly_linked_list.delete_at_index(1); // now the linked list is 1->3
    println!("{:?}", my_doubly_linked_list);
    my_doubly_linked_list.get(1);
    println!("{:?}", my_doubly_linked_list);
}
