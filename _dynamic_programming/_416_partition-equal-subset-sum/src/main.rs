/*
 * @lc app=leetcode id=416 lang=rust
 *
 * [416] Partition Equal Subset Sum
 *
 * https://leetcode.com/problems/partition-equal-subset-sum/description/
 *
 * algorithms
 * Medium (46.65%)
 * Likes:    9890
 * Dislikes: 171
 * Total Accepted:    593K
 * Total Submissions: 1.3M
 * Testcase Example:  '[1,5,11,5]'
 *
 * Given an integer array nums, return true if you can partition the array into
 * two subsets such that the sum of the elements in both subsets is equal or
 * false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,5,11,5]
 * Output: true
 * Explanation: The array can be partitioned as [1, 5, 5] and [11].
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,3,5]
 * Output: false
 * Explanation: The array cannot be partitioned into equal sum subsets.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 200
 * 1 <= nums[i] <= 100
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
        }

        if sum % 2 == 1 {
            return false;
        }

        Self::subset_sum(&nums, (sum / 2) as usize)
    }

    fn subset_sum(nums: &Vec<i32>, target: usize) -> bool {
        let mut dp = vec![vec![-1; target + 1]; nums.len() + 1];
        let ans = Self::helper(nums.len(), target, &mut dp, &nums);
        if ans == 1 {
            true
        } else {
            false
        }
    }

    fn helper(i: usize, j: usize, dp: &mut Vec<Vec<i32>>, nums: &Vec<i32>) -> i32 {
        if dp[i][j] == -1 {
            if j == 0 {
                dp[i][j] = 1;
            } else if i == 0 {
                dp[i][j] = 0;
            } else {
                dp[i][j] = Self::helper(i - 1, j, dp, nums);
                if dp[i][j] == 0 && j >= nums[i - 1] as usize {
                    dp[i][j] = Self::helper(i - 1, j - nums[i - 1] as usize, dp, nums)
                }
            }
        }

        dp[i][j]
    }

    pub fn can_partition_iteraction(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
        }

        if sum % 2 == 1 {
            return false;
        }

        Self::subset_sum_iteraction(&nums, (sum / 2) as usize)
    }

    fn subset_sum_iteraction(nums: &Vec<i32>, target: usize) -> bool {
        let mut dp = vec![vec![false; target + 1]; nums.len() + 1];
        for i in 0..=nums.len() {
            dp[i][0] = true;
        }

        for i in 1..=nums.len() {
            for j in 1..=target {
                dp[i][j] = dp[i - 1][j];
                if !dp[i][j] && j >= nums[i - 1] as usize {
                    dp[i][j] = dp[i - 1][j - nums[i - 1] as usize];
                }
            }
        }
        dp[nums.len()][target]
    }

    pub fn can_partition_iteraction_optimization(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
        }

        if sum % 2 == 1 {
            return false;
        }

        Self::subset_sum_iteraction_optimization(&nums, (sum / 2) as usize)
    }

    fn subset_sum_iteraction_optimization(nums: &Vec<i32>, target: usize) -> bool {
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for i in 0..nums.len() {
            for j in (nums[i] as usize..=target).rev() {
                if !dp[j] {
                    dp[j] = dp[j - nums[i] as usize];
                }
            }
        }
        dp[target]
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(Solution::can_partition_iteraction(vec![1, 5, 11, 5]), true);
    assert_eq!(
        Solution::can_partition_iteraction_optimization(vec![1, 5, 11, 5]),
        true
    );
}
