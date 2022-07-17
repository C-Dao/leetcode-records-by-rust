/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        (i + 1) as i32
    }
}
// @lc code=end

fn main() {
    let mut nums = vec![1, 1, 2];
    println!("{}", Solution::remove_duplicates(&mut nums));
}
