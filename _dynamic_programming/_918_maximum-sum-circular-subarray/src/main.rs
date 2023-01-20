/*
 * @lc app=leetcode id=918 lang=rust
 *
 * [918] Maximum Sum Circular Subarray
 *
 * https://leetcode.com/problems/maximum-sum-circular-subarray/description/
 *
 * algorithms
 * Medium (37.58%)
 * Likes:    5105
 * Dislikes: 221
 * Total Accepted:    176.7K
 * Total Submissions: 431.2K
 * Testcase Example:  '[1,-2,3,-2]'
 *
 * Given a circular integer array nums of length n, return the maximum possible
 * sum of a non-empty subarray of nums.
 *
 * A circular array means the end of the array connects to the beginning of the
 * array. Formally, the next element of nums[i] is nums[(i + 1) % n] and the
 * previous element of nums[i] is nums[(i - 1 + n) % n].
 *
 * A subarray may only include each element of the fixed buffer nums at most
 * once. Formally, for a subarray nums[i], nums[i + 1], ..., nums[j], there
 * does not exist i <= k1, k2 <= j with k1 % n == k2 % n.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,-2,3,-2]
 * Output: 3
 * Explanation: Subarray [3] has maximum sum 3.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [5,-3,5]
 * Output: 10
 * Explanation: Subarray [5,5] has maximum sum 5 + 5 = 10.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [-3,-2,-3]
 * Output: -2
 * Explanation: Subarray [-2] has maximum sum -2.
 *
 *
 *
 * Constraints:
 *
 *
 * n == nums.length
 * 1 <= n <= 3 * 10^4
 * -3 * 10^4 <= nums[i] <= 3 * 10^4
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut max_sum, mut min_sum, mut total_sum, mut cur_max, mut cur_min) = (i32::MIN, i32::MAX, 0, 0, 0);

        for i in 0..nums.len() {
            cur_max = i32::max(nums[i], cur_max + nums[i]);
            max_sum = i32::max(cur_max, max_sum);

            cur_min = i32::min(nums[i], cur_min + nums[i]);
            min_sum = i32::min(cur_min, min_sum);

            total_sum += nums[i];
        }

        // edge_case: All numbers in this array are negative
        if total_sum == min_sum {
            max_sum
        } else {
            i32::max(max_sum, total_sum - min_sum)
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
}
