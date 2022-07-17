/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                if j != i {
                    nums[i] = nums[j];
                    nums[j] = 0;
                }
                i += 1;
            }
        }
    }
}
// @lc code=end

fn main() {
    let mut nums_1 = vec![0, 1, 0, 3, 12];
    let mut nums_2 = vec![1];
    Solution::move_zeroes(&mut nums_1);
    Solution::move_zeroes(&mut nums_2);
    println!("{:?}", nums_1);
    println!("{:?}", nums_2);
}
