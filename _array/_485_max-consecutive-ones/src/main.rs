/*
 * @lc app=leetcode id=485 lang=rust
 *
 * [485] Max Consecutive Ones
 *
 * https://leetcode.com/problems/max-consecutive-ones/description/
 *
 * algorithms
 * Easy (55.43%)
 * Likes:    2925
 * Dislikes: 404
 * Total Accepted:    695.2K
 * Total Submissions: 1.3M
 * Testcase Example:  '[1,1,0,1,1,1]'
 *
 * Given a binary array nums, return the maximum number of consecutive 1's in
 * the array.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [1,1,0,1,1,1]
 * Output: 3
 * Explanation: The first two digits or the last three digits are consecutive
 * 1s. The maximum number of consecutive 1s is 3.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [1,0,1,1,0,1]
 * Output: 2
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 10^5
 * nums[i] is either 0 or 1.
 * 
 * 
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
