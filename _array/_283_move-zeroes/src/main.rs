/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 *
 * https://leetcode.com/problems/move-zeroes/description/
 *
 * algorithms
 * Easy (60.93%)
 * Likes:    9955
 * Dislikes: 254
 * Total Accepted:    1.8M
 * Total Submissions: 2.9M
 * Testcase Example:  '[0,1,0,3,12]'
 *
 * Given an integer array nums, move all 0's to the end of it while maintaining
 * the relative order of the non-zero elements.
 * 
 * Note that you must do this in-place without making a copy of the array.
 * 
 * 
 * Example 1:
 * Input: nums = [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 * Example 2:
 * Input: nums = [0]
 * Output: [0]
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 10^4
 * -2^31 <= nums[i] <= 2^31 - 1
 * 
 * 
 * 
 * Follow up: Could you minimize the total number of operations done?
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
