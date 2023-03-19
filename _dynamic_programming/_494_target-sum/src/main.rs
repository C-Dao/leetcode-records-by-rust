/*
 * @lc app=leetcode id=494 lang=rust
 *
 * [494] Target Sum
 *
 * https://leetcode.com/problems/target-sum/description/
 *
 * algorithms
 * Medium (45.36%)
 * Likes:    8836
 * Dislikes: 311
 * Total Accepted:    440.3K
 * Total Submissions: 965K
 * Testcase Example:  '[1,1,1,1,1]\n3'
 *
 * You are given an integer array nums and an integer target.
 *
 * You want to build an expression out of nums by adding one of the symbols '+'
 * and '-' before each integer in nums and then concatenate all the
 * integers.
 *
 *
 * For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1
 * and concatenate them to build the expression "+2-1".
 *
 *
 * Return the number of different expressions that you can build, which
 * evaluates to target.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,1,1,1,1], target = 3
 * Output: 5
 * Explanation: There are 5 ways to assign symbols to make the sum of nums be
 * target 3.
 * -1 + 1 + 1 + 1 + 1 = 3
 * +1 - 1 + 1 + 1 + 1 = 3
 * +1 + 1 - 1 + 1 + 1 = 3
 * +1 + 1 + 1 - 1 + 1 = 3
 * +1 + 1 + 1 + 1 - 1 = 3
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1], target = 1
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 20
 * 0 <= nums[i] <= 1000
 * 0 <= sum(nums[i]) <= 1000
 * -1000 <= target <= 1000
 *
 *
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut sum: i32 = 0;
        for i in 0..nums.len() {
            sum += nums[i];
        }

        if (sum + target) % 2 == 1 || sum - target < 0 || target + sum < 0 {
            return 0;
        }

        Self::subset_sum(&nums, ((sum + target) / 2) as usize)
    }

    fn subset_sum(nums: &Vec<i32>, target: usize) -> i32 {
        let mut dp = vec![vec![0; target + 1]; nums.len() + 1];

        for i in 0..=nums.len() {
            dp[i][0] = 1;
        }

        for i in 1..=nums.len() {
            for j in 0..=target {
                dp[i][j] = dp[i - 1][j];
                if j >= nums[i - 1] as usize {
                    dp[i][j] += dp[i - 1][j - nums[i - 1] as usize];
                }
            }
        }
        dp[nums.len()][target]
    }

    pub fn find_target_sum_ways_optimization(nums: Vec<i32>, target: i32) -> i32 {
        let mut sum: i32 = 0;
        for i in 0..nums.len() {
            sum += nums[i];
        }

        if (sum + target) % 2 == 1 || sum - target < 0 || target + sum < 0 {
            return 0;
        }

        Self::subset_sum_optimization(&nums, ((sum + target) / 2) as usize)
    }

    fn subset_sum_optimization(nums: &Vec<i32>, target: usize) -> i32 {
        let mut dp = vec![0; target + 1];
        dp[0] = 1;

        for i in 0..nums.len() {
            for j in (nums[i] as usize..=target).rev() {
                if j >= nums[i] as usize {
                    dp[j] += dp[j - nums[i] as usize];
                }
            }
        }
        dp[target]
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    assert_eq!(
        Solution::find_target_sum_ways(vec![0, 0, 0, 0, 0, 0, 0, 0, 1], 1),
        256
    );
    assert_eq!(Solution::find_target_sum_ways(vec![1, 2, 1], 0), 2);
    assert_eq!(Solution::find_target_sum_ways(vec![100], -200), 0);
    assert_eq!(
        Solution::find_target_sum_ways_optimization(vec![1, 1, 1, 1, 1], 3),
        5
    );
    assert_eq!(
        Solution::find_target_sum_ways_optimization(vec![0, 0, 0, 0, 0, 0, 0, 0, 1], 1),
        256
    );
    assert_eq!(
        Solution::find_target_sum_ways_optimization(vec![1, 2, 1], 0),
        2
    );
    assert_eq!(Solution::find_target_sum_ways(vec![100], -200), 0);

}
