/*
 * @lc app=leetcode id=491 lang=rust
 *
 * [491] Non-decreasing Subsequences
 *
 * https://leetcode.com/problems/non-decreasing-subsequences/description/
 *
 * algorithms
 * Medium (58.6%)
 * Likes:    1866
 * Dislikes: 870
 * Total Accepted:    129K
 * Total Submissions: 220.2K
 * Testcase Example:  '[4,6,7,7]'
 *
 * Given an integer array nums, return all the different possible non-decreasing subsequences
 * of the given array with at least two elements. You may return the answer in any order.
 *
 *
 *
 *
 * Example 1:
 *
 * Input: nums = [4,6,7,7]
 * Output: [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
 *
 *
 * Example 2:
 *
 * Input: nums = [4,4,3,2,1]
 * Output: [[4,4]]
 *
 *
 *
 *
 * Constraints:
 * 1 <= nums.length <= 15
 * -100 <= nums[i] <= 100
 *
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut cur_sub_sequence = vec![];
        Self::dfs(0, &nums, &mut ans, &mut cur_sub_sequence);
        ans
    }

    fn dfs(cur: usize, nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>, cur_sub_sequence: &mut Vec<i32>) {
        if cur == nums.len() && cur_sub_sequence.len() >= 2 {
            ans.push(cur_sub_sequence.clone());
            return;
        }

        if cur == nums.len() {
            return;
        }

        if cur_sub_sequence.is_empty() || nums[cur] >= *cur_sub_sequence.last().unwrap() {
            cur_sub_sequence.push(nums[cur]);
            Self::dfs(cur + 1, nums, ans, cur_sub_sequence);
            cur_sub_sequence.pop();
        }

        if cur_sub_sequence.is_empty() || nums[cur] != *cur_sub_sequence.last().unwrap() {
            Self::dfs(cur + 1, nums, ans, cur_sub_sequence);
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::find_subsequences(vec![4, 6, 7, 7]),
        vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7]
        ]
    );
}
