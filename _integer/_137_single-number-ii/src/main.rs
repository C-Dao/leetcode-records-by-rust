/*
 * @lc app=leetcode id=137 lang=rust
 *
 * [137] Single Number II
 *
 * https://leetcode.com/problems/single-number-ii/description/
 *
 * algorithms
 * Medium (57.03%)
 * Likes:    4984
 * Dislikes: 523
 * Total Accepted:    384.7K
 * Total Submissions: 664.2K
 * Testcase Example:  '[2,2,3,2]'
 *
 * Given an integer array nums where every element appears three times except
 * for one, which appears exactly once. Find the single element and return it.
 *
 * You must implement a solution with a linear runtime complexity and use only
 * constant extra space.
 *
 *
 * Example 1:
 * Input: nums = [2,2,3,2]
 * Output: 3
 * Example 2:
 * Input: nums = [0,1,0,1,0,1,99]
 * Output: 99
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 3 * 10^4
 * -2^31 <= nums[i] <= 2^31 - 1
 * Each element in nums appears exactly three times except for one element
 * which appears once.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bit_sums = vec![0; 32];
        let mut ans = 0;

        for num in nums {
            for i in 0..32 {
                bit_sums[i] += (num >> (31 - i)) & 1;
            }
        }

        for i in 0..32 {
            ans = (ans << 1) + bit_sums[i] % 3;
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
}
