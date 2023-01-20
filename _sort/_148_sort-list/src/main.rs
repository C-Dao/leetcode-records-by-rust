/*
 * @lc app=leetcode id=148 lang=rust
 *
 * [148] Sort List
 *
 * https://leetcode.com/problems/sort-list/description/
 *
 * algorithms
 * Medium (52.86%)
 * Likes:    8891
 * Dislikes: 271
 * Total Accepted:    584.3K
 * Total Submissions: 1.1M
 * Testcase Example:  '[4,2,1,3]'
 *
 * Given the head of a linked list, return the list after sorting it in
 * ascending order.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: head = [4,2,1,3]
 * Output: [1,2,3,4]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: head = [-1,5,3,4,0]
 * Output: [-1,0,3,4,5]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: head = []
 * Output: []
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the list is in the range [0, 5 * 10^4].
 * -10^5 <= Node.val <= 10^5
 * 
 * 
 * 
 * Follow up: Can you sort the linked list in O(n logn) time and O(1) memory
 * (i.e. constant space)?
 * 
 */

 use data_structures::*;
 use data_structure_marcos::*;

 struct Solution{}


// @lc code=start
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.unwrap().next.is_none(){
            return head;
        }

        let mut head_1 = head;
        let mut head_2 = Self::split(head);
    }

    fn split(&mut head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        let (mut slow, mut fast) = 
    }
}
// @lc code=end


fn main() {
    println!("Hello, world!");
}
