/*
 * @lc app=leetcode id=160 lang=rust
 *
 * [160] Intersection of Two Linked Lists
 *
 * https://leetcode.com/problems/intersection-of-two-linked-lists/description/
 *
 * algorithms
 * Easy (51.91%)
 * Likes:    10466
 * Dislikes: 977
 * Total Accepted:    1.1M
 * Total Submissions: 2M
 * Testcase Example:  '8\n[4,1,8,4,5]\n[5,6,1,8,4,5]\n2\n3'
 *
 * Given the heads of two singly linked-lists headA and headB, return the node
 * at which the two lists intersect. If the two linked lists have no
 * intersection at all, return null.
 * 
 * For example, the following two linked lists begin to intersect at node c1:
 * 
 * The test cases are generated such that there are no cycles anywhere in the
 * entire linked structure.
 * 
 * Note that the linked lists must retain their original structure after the
 * function returns.
 * 
 * Custom Judge:
 * 
 * The inputs to the judge are given as follows (your program is not given
 * these inputs):
 * 
 * 
 * intersectVal - The value of the node where the intersection occurs. This is
 * 0 if there is no intersected node.
 * listA - The first linked list.
 * listB - The second linked list.
 * skipA - The number of nodes to skip ahead in listA (starting from the head)
 * to get to the intersected node.
 * skipB - The number of nodes to skip ahead in listB (starting from the head)
 * to get to the intersected node.
 * 
 * 
 * The judge will then create the linked structure based on these inputs and
 * pass the two heads, headA and headB to your program. If you correctly return
 * the intersected node, then your solution will be accepted.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5], skipA =
 * 2, skipB = 3
 * Output: Intersected at '8'
 * Explanation: The intersected node's value is 8 (note that this must not be 0
 * if the two lists intersect).
 * From the head of A, it reads as [4,1,8,4,5]. From the head of B, it reads as
 * [5,6,1,8,4,5]. There are 2 nodes before the intersected node in A; There are
 * 3 nodes before the intersected node in B.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: intersectVal = 2, listA = [1,9,1,2,4], listB = [3,2,4], skipA = 3,
 * skipB = 1
 * Output: Intersected at '2'
 * Explanation: The intersected node's value is 2 (note that this must not be 0
 * if the two lists intersect).
 * From the head of A, it reads as [1,9,1,2,4]. From the head of B, it reads as
 * [3,2,4]. There are 3 nodes before the intersected node in A; There are 1
 * node before the intersected node in B.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: intersectVal = 0, listA = [2,6,4], listB = [1,5], skipA = 3, skipB =
 * 2
 * Output: No intersection
 * Explanation: From the head of A, it reads as [2,6,4]. From the head of B, it
 * reads as [1,5]. Since the two lists do not intersect, intersectVal must be
 * 0, while skipA and skipB can be arbitrary values.
 * Explanation: The two lists do not intersect, so return null.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes of listA is in the m.
 * The number of nodes of listB is in the n.
 * 1 <= m, n <= 3 * 10^4
 * 1 <= Node.val <= 10^5
 * 0 <= skipA < m
 * 0 <= skipB < n
 * intersectVal is 0 if listA and listB do not intersect.
 * intersectVal == listA[skipA] == listB[skipB] if listA and listB
 * intersect.
 * 
 * 
 * 
 * Follow up: Could you write a solution that runs in O(m + n) time and use
 * only O(1) memory?
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
    pub fn get_intersection_node(
        head_1: Option<Rc<RefCell<ListNode>>>,
        head_2: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        let mut pointer_one = match head_1 {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };
        let mut pointer_two = match head_2 {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };

        while pointer_one.as_ptr() != pointer_two.as_ptr() {
            let mut pointer_one_next = pointer_one.borrow().next.clone();
            let mut pointer_two_next = pointer_two.borrow().next.clone();

            if pointer_one_next.is_none() && pointer_two_next.is_none() {
                return None;
            }

            if pointer_one_next.is_none() {
                pointer_one_next = head_2.clone();
                pointer_one = pointer_one_next.unwrap().clone();
            } else if let Some(ref node) = pointer_one_next {
                pointer_one = Rc::clone(node);
            }

            if pointer_two_next.is_none() {
                pointer_two_next = head_1.clone();
                pointer_two = pointer_two_next.unwrap().clone();
            } else if let Some(ref node) = pointer_two_next {
                pointer_two = Rc::clone(node);
            }
        }
        Some(pointer_one)
    }

    pub fn get_intersection_node_hash_map_edition(
        head_1: Option<Rc<RefCell<ListNode>>>,
        head_2: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        let mut set = HashSet::new();

        let mut ptr = match head_1 {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };

        while set.insert(ptr.as_ptr()) {
            let next = match ptr.borrow().next {
                Some(ref node) => Rc::clone(node),
                None => break,
            };
            ptr = next;
        }

        ptr = match head_2 {
            Some(ref node) => Rc::clone(node),
            None => return None,
        };
        while !set.contains(&ptr.as_ptr()) {
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
    let a = Rc::new(RefCell::new(ListNode::new(4)));
    let b = Rc::new(RefCell::new(ListNode::new(1)));
    let c = Rc::new(RefCell::new(ListNode::new(8)));
    let d = Rc::new(RefCell::new(ListNode::new(4)));
    let e = Rc::new(RefCell::new(ListNode::new(5)));
    let f = Rc::new(RefCell::new(ListNode::new(5)));
    let g = Rc::new(RefCell::new(ListNode::new(6)));
    let h = Rc::new(RefCell::new(ListNode::new(1)));

    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());
    c.borrow_mut().next = Some(d.clone());
    d.borrow_mut().next = Some(e.clone());

    f.borrow_mut().next = Some(g.clone());
    g.borrow_mut().next = Some(h.clone());
    h.borrow_mut().next = Some(c.clone());

    assert_eq!(
        Solution::get_intersection_node(Some(a.clone()), Some(f.clone()))
            .unwrap()
            .as_ptr(),
        c.clone().as_ptr()
    );
    assert_eq!(
        Solution::get_intersection_node_hash_map_edition(Some(a.clone()), Some(f.clone()))
            .unwrap()
            .as_ptr(),
        c.clone().as_ptr()
    );

    let a = Rc::new(RefCell::new(ListNode::new(2)));
    let b = Rc::new(RefCell::new(ListNode::new(6)));
    let c = Rc::new(RefCell::new(ListNode::new(4)));
    let d = Rc::new(RefCell::new(ListNode::new(1)));
    let e = Rc::new(RefCell::new(ListNode::new(5)));

    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());

    d.borrow_mut().next = Some(e.clone());

    assert_eq!(
        Solution::get_intersection_node(Some(a.clone()), Some(d.clone())),
        None
    );
    assert_eq!(
        Solution::get_intersection_node_hash_map_edition(Some(a.clone()), Some(d.clone())),
        None
    );

    let a = Rc::new(RefCell::new(ListNode::new(1)));
    let b = Rc::new(RefCell::new(ListNode::new(9)));
    let c = Rc::new(RefCell::new(ListNode::new(1)));
    let d = Rc::new(RefCell::new(ListNode::new(2)));
    let e = Rc::new(RefCell::new(ListNode::new(4)));
    let f = Rc::new(RefCell::new(ListNode::new(3)));

    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());
    c.borrow_mut().next = Some(d.clone());
    d.borrow_mut().next = Some(e.clone());

    f.borrow_mut().next = Some(d.clone());

    assert_eq!(
        Solution::get_intersection_node(Some(a.clone()), Some(f.clone()))
            .unwrap()
            .as_ptr(),
        d.clone().as_ptr()
    );
    assert_eq!(
        Solution::get_intersection_node_hash_map_edition(Some(a.clone()), Some(f.clone()))
            .unwrap()
            .as_ptr(),
        d.clone().as_ptr()
    );
}
