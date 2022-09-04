/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
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
