/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 *
 * https://leetcode.com/problems/pascals-triangle-ii/description/
 *
 * algorithms
 * Easy (58.36%)
 * Likes:    3090
 * Dislikes: 279
 * Total Accepted:    582.5K
 * Total Submissions: 985.5K
 * Testcase Example:  '3'
 *
 * Given an integer rowIndex, return the rowIndex^th (0-indexed) row of the
 * Pascal's triangle.
 * 
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it as shown:
 * 
 * 
 * Example 1:
 * Input: rowIndex = 3
 * Output: [1,3,3,1]
 * Example 2:
 * Input: rowIndex = 0
 * Output: [1]
 * Example 3:
 * Input: rowIndex = 1
 * Output: [1,1]
 * 
 * 
 * Constraints:
 * 
 * 
 * 0 <= rowIndex <= 33
 * 
 * 
 * 
 * Follow up: Could you optimize your algorithm to use only O(rowIndex) extra
 * space?
 * 
 */
struct Solution {}

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row_values = vec![];
        for row in 0..=row_index as usize {
            row_values.resize(row + 1, 1);
            for col in (1..row as usize).rev() {
                row_values[col] += row_values[col - 1];
            }
        }
        row_values
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
}
