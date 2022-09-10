/*
 * @lc app=leetcode id=75 lang=rust
 *
 * [75] Sort Colors
 *
 * https://leetcode.com/problems/sort-colors/description/
 *
 * algorithms
 * Medium (55.78%)
 * Likes:    11918
 * Dislikes: 448
 * Total Accepted:    1.2M
 * Total Submissions: 2.1M
 * Testcase Example:  '[2,0,2,1,1,0]'
 *
 * Given an array nums with n objects colored red, white, or blue, sort them
 * in-place so that objects of the same color are adjacent, with the colors in
 * the order red, white, and blue.
 * 
 * We will use the integers 0, 1, and 2 to represent the color red, white, and
 * blue, respectively.
 * 
 * You must solve this problem without using the library's sort function.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [2,0,1]
 * Output: [0,1,2]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * n == nums.length
 * 1 <= n <= 300
 * nums[i] is either 0, 1, or 2.
 * 
 * 
 * 
 * Follow up: Could you come up with a one-pass algorithm using only constant
 * extra space?
 * 
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut p0, mut p2, mut i) = (0, nums.len() - 1, 0);
        while i <= p2 && p2 != 0 {
            while p2 != 0 && i <= p2 && nums[i] == 2 {
                nums.swap(i, p2);
                p2 -= 1;
            }
            while i >= p0 && nums[i] == 0 {
                nums.swap(i, p0);
                p0 += 1;
            }

            i += 1;
        }
    }
}
// @lc code=end

fn main() {
    let mut nums = vec![2, 2];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![2, 2]);
}
