/*
 * @lc app=leetcode id=561 lang=rust
 *
 * [561] Array Partition
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.iter().step_by(2).sum()
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
}
