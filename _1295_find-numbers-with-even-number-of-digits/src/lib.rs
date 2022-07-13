/*
 * @lc app=leetcode id=1295 lang=rust
 *
 * [1295] Find Numbers with Even Number of Digits
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            result += if num.to_string().len() % 2 == 0 { 1 } else { 0 };
        }
        result
    }
}
// @lc code=end

fn main() {
    Solution::find_numbers(vec![12, 345, 2, 6, 7896]);
}
