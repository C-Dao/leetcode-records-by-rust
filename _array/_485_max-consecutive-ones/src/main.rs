/*
 * @lc app=leetcode id=485 lang=rust
 *
 * [485] Max Consecutive Ones
 */
struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut cur_len = 0;

        for i in nums {
            if i == 1 {
                cur_len += 1;
            } else if i != 1 {
                max = if cur_len > max { cur_len } else { max };
                cur_len = 0;
            }
        }
        if cur_len > max {
            cur_len
        } else {
            max
        }
    }
}
// @lc code=end

fn main() {
    Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]);
}
