/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 *
 * https://leetcode.com/problems/maximum-subarray/description/
 *
 * algorithms
 * Medium (49.69%)
 * Likes:    26226
 * Dislikes: 1180
 * Total Accepted:    2.8M
 * Total Submissions: 5.7M
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * Given an integer array nums, find the subarray which has the largest sum and
 * return its sum.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1]
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [5,4,-1,7,8]
 * Output: 23
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 *
 *
 *
 * Follow up: If you have figured out the O(n) solution, try coding another
 * solution using the divide and conquer approach, which is more subtle.
 *
 */

struct RangeState {
    l_max_sum: i32,
    r_max_sum: i32,
    max_sum: i32,
    range_sum: i32,
}

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_sub_array(mut nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        for i in 1..nums.len() {
            nums[i] = i32::max(nums[i] + nums[i - 1], nums[i]);
            max = i32::max(nums[i], max);
        }

        max
    }

    pub fn max_sub_array_2(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        Self::get(&nums, 0, len - 1).max_sum
    }

    fn push_up(l_range: RangeState, r_range: RangeState) -> RangeState {
        RangeState {
            l_max_sum: i32::max(l_range.range_sum + r_range.l_max_sum, l_range.l_max_sum),
            r_max_sum: i32::max(r_range.range_sum + l_range.r_max_sum, r_range.r_max_sum),
            max_sum: i32::max(
                i32::max(l_range.max_sum, r_range.max_sum),
                l_range.r_max_sum + r_range.l_max_sum,
            ),
            range_sum: l_range.range_sum + r_range.range_sum,
        }
    }

    fn get(nums: &Vec<i32>, l: usize, r: usize) -> RangeState {
        if l == r {
            return RangeState {
                l_max_sum: nums[l],
                r_max_sum: nums[l],
                max_sum: nums[l],
                range_sum: nums[l],
            };
        }
        let m = (l + r) >> 1;
        let l_range_state_sub = Self::get(nums, l, m);
        let r_range_state_sub = Self::get(nums, m + 1, r);
        Self::push_up(l_range_state_sub, r_range_state_sub)
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(Solution::max_sub_array_2(vec![5, 4, -1, 7, 8]), 23);
    assert_eq!(Solution::max_sub_array_2(vec![1]), 1);
    assert_eq!(
        Solution::max_sub_array_2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
}
