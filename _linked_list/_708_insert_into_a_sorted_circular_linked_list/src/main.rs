/*
* @lc app=leetcode id=708 lang=rust
*
* [708] Insert into a Sorted Circular Linked List
*
* https://leetcode.cn/problems/insert-into-a-sorted-circular-linked-list/description/
*
* algorithms
* Medium
* Likes:
* Dislikes:
* Total Accepted:
* Total Submissions:
* Testcase Example:
*
* Given a node from a cyclic linked list which is sorted in ascending order,
* write a function to insert a value into the list such that it remains a cyclic
* sorted list. The given node can be a reference to any single node in the list,
* and may not be necessarily the smallest value in the cyclic list.
*
* If there are multiple suitable places for insertion, you may choose any place to
* insert the new value. After the insertion, the cyclic list should remain sorted.

* If the list is empty (i.e., given node is null), you should create a new single
* cyclic list and return the reference to that single node. Otherwise, you should
* return the original given node.
*
* The following example may help you understand the problem better:
*
*
* Example 1:
*
*
* Input: head = [3,4,1], insertVal = 2
* Output: [3,4,1,2]
*
*
* Example 2:
*
*
* Input: head = [], insertVal = 1
* Output: [1]
*
*
* Example 3:
*
*
* Input: head = [1], insertVal = 0
* Output: [1,0]
*
*
*
* Constraints:
*
*
* 0 <= Number of Nodes <= 5 * 10^4
* -10^6 <= Node.val <= 10^6
* -10^6 <= insertVal <= 10^6
*
*
*
*
*/

// Definition for singly-linked list.
use std::fmt::{Debug, Error, Formatter};
use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

// @lc code=start

impl Solution {
    pub fn insert(
        mut head: Option<Rc<RefCell<ListNode>>>,
        insert_val: i32,
    ) -> Option<Rc<RefCell<ListNode>>> {
        if head.is_none() {
            return Some(Rc::new(RefCell::new(ListNode::new(insert_val))));
        };

        if head.as_ref().unwrap().borrow().next == head {
            let mut node = Some(Rc::new(RefCell::new(ListNode::new(insert_val))));
            node.as_mut().unwrap().borrow_mut().next = head.clone();
            head.as_mut().unwrap().borrow_mut().next = node;
            return head;
        };

        let (mut cur, mut next, mut biggest) = (
            head.clone(),
            head.as_ref().unwrap().borrow().next.clone(),
            head.clone(),
        );
        let mut node = Some(Rc::new(RefCell::new(ListNode::new(insert_val))));

        while (cur.as_ref().unwrap().borrow().val > node.as_ref().unwrap().borrow().val
            || next.as_ref().unwrap().borrow().val < node.as_ref().unwrap().borrow().val)
            && next != head
        {
            cur = next.clone();
            let next_next = next.as_ref().unwrap().borrow().next.clone();
            next = next_next;

            if cur.as_ref().unwrap().borrow().val >= biggest.as_ref().unwrap().borrow().val {
                biggest = cur.clone();
            };
        }

        if cur.as_ref().unwrap().borrow().val <= node.as_ref().unwrap().borrow().val
            && next.as_ref().unwrap().borrow().val >= node.as_ref().unwrap().borrow().val
        {
            node.as_mut().unwrap().borrow_mut().next = next;
            cur.as_mut().unwrap().borrow_mut().next = node;
        } else {
            node.as_mut().unwrap().borrow_mut().next =
                biggest.as_ref().unwrap().borrow_mut().next.take();
            biggest.as_mut().unwrap().borrow_mut().next = node;
        }

        head
    }
}

// @lc code=end

struct ListNodeList {
    head: Option<Rc<RefCell<ListNode>>>,
}

impl Debug for ListNodeList {
    fn fmt(&self, f: &'_ mut Formatter) -> Result<(), Error> {
        let mut node = self.head.clone();
        while node.is_some() {
            writeln!(
                f,
                "{:?} == next ==>",
                node.as_mut().unwrap().borrow_mut().val
            )?;
            let next = node.as_mut().unwrap().borrow_mut().next.take();
            node = next;
            if node == self.head {
                break;
            }
        }
        return Ok(());
    }
}

fn main() {
    let a = Rc::new(RefCell::new(ListNode::new(3)));
    let b = Rc::new(RefCell::new(ListNode::new(4)));
    let c = Rc::new(RefCell::new(ListNode::new(1)));

    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());
    c.borrow_mut().next = Some(a.clone());

    println!(
        "{:?}",
        ListNodeList {
            head: Solution::insert(Some(a), 2)
        }
    );
}
