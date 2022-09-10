/*
 * @lc app=leetcode id=142 lang=rust
 *
 * [142] Linked List Cycle II
 *
 * https://leetcode.com/problems/linked-list-cycle-ii/description/
 *
 * algorithms
 * Medium (45.02%)
 * Likes:    8170
 * Dislikes: 555
 * Total Accepted:    749.6K
 * Total Submissions: 1.7M
 * Testcase Example:  '[3,2,0,-4]\n1'
 *
 * Given the head of a linked list, return the node where the cycle begins. If
 * there is no cycle, return null.
 * 
 * There is a cycle in a linked list if there is some node in the list that can
 * be reached again by continuously following the next pointer. Internally, pos
 * is used to denote the index of the node that tail's next pointer is
 * connected to (0-indexed). It is -1 if there is no cycle. Note that pos is
 * not passed as a parameter.
 * 
 * Do not modify the linked list.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: head = [3,2,0,-4], pos = 1
 * Output: tail connects to node index 1
 * Explanation: There is a cycle in the linked list, where tail connects to the
 * second node.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: head = [1,2], pos = 0
 * Output: tail connects to node index 0
 * Explanation: There is a cycle in the linked list, where tail connects to the
 * first node.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: head = [1], pos = -1
 * Output: no cycle
 * Explanation: There is no cycle in the linked list.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of the nodes in the list is in the range [0, 10^4].
 * -10^5 <= Node.val <= 10^5
 * pos is -1 or a valid index in the linked-list.
 * 
 * 
 * 
 * Follow up: Can you solve it using O(1) (i.e. constant) memory?
 * 
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

        Some(ptr)
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
