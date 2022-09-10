/*
 * @lc app=leetcode id=328 lang=rust
 *
 * [328] Odd Even Linked List
 *
 * https://leetcode.com/problems/odd-even-linked-list/description/
 *
 * algorithms
 * Medium (59.78%)
 * Likes:    5832
 * Dislikes: 390
 * Total Accepted:    564.1K
 * Total Submissions: 941.6K
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * Given the head of a singly linked list, group all the nodes with odd indices
 * together followed by the nodes with even indices, and return the reordered
 * list.
 * 
 * The first node is considered odd, and the second node is even, and so on.
 * 
 * Note that the relative order inside both the even and odd groups should
 * remain as it was in the input.
 * 
 * You must solve the problem in O(1) extra space complexity and O(n) time
 * complexity.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: head = [1,2,3,4,5]
 * Output: [1,3,5,2,4]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: head = [2,1,3,5,6,4,7]
 * Output: [2,3,6,7,1,5,4]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the linked list is in the range [0, 10^4].
 * -10^6 <= Node.val <= 10^6
 * 
 * 
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_dummy_head = Box::new(ListNode {
            val: -1,
            next: None,
        });
        let mut even_dummy_head = Box::new(ListNode {
            val: -1,
            next: None,
        });

        let mut even_cur = even_dummy_head.as_mut();
        let mut odd_cur = odd_dummy_head.as_mut();

        while head.is_some() {
            let odd = match head.take() {
                Some(node) => node,
                None => break,
            };

            odd_cur.next = Some(odd);
            odd_cur = odd_cur.next.as_mut().unwrap();

            let even = match odd_cur.next.take() {
                Some(node) => node,
                None => break,
            };

            even_cur.next = Some(even);
            even_cur = even_cur.next.as_mut().unwrap();

            head = even_cur.next.take();
        }

        odd_cur.next = even_dummy_head.next;

        odd_dummy_head.next
    }
}
// @lc code=end

fn main() {
    let mut a = Box::new(ListNode::new(1));
    let mut b = Box::new(ListNode::new(2));
    let mut c = Box::new(ListNode::new(3));
    let mut d = Box::new(ListNode::new(4));
    let e = Box::new(ListNode::new(5));

    d.next = Some(e);
    c.next = Some(d);
    b.next = Some(c);
    a.next = Some(b);

    println!("{:?}", Solution::odd_even_list(Some(a)));
}
