/*
 * @lc app=leetcode id=1004 lang=rust
 *
 * [1004] Max Consecutive Ones III
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut lo, mut hi, mut count) = (0, 0, k);
        while hi < nums.len() {
            if nums[hi] == 0 {
                count -= 1;
            }
            if count < 0 {
                if nums[lo] == 0 {
                    count += 1;
                }
                lo += 1;
            }
            hi += 1;
        }
        (hi - lo) as i32
    }
}
// @lc code=end

fn main() {
    let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let k = 2;
    println!("{:?}", Solution::longest_ones(nums, k));
}
