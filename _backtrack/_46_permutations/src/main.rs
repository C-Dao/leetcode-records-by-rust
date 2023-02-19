/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 *
 * https://leetcode.com/problems/permutations/description/
 *
 * algorithms
 * Medium (73.29%)
 * Likes:    14193
 * Dislikes: 240
 * Total Accepted:    1.5M
 * Total Submissions: 1.9M
 * Testcase Example:  '[1,2,3]'
 *
 * Given an array nums of distinct integers, return all the possible
 * permutations. You can return the answer in any order.
 *
 *
 * Example 1:
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * Example 2:
 * Input: nums = [0,1]
 * Output: [[0,1],[1,0]]
 * Example 3:
 * Input: nums = [1]
 * Output: [[1]]
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 6
 * -10 <= nums[i] <= 10
 * All the integers of nums are unique.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(0, &mut nums, &mut ans);

        ans
    }

    fn dfs(index: usize, nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            ans.push(nums.clone());
            return;
        };

        for i in index..nums.len() {
            nums.swap(index, i);
            Self::dfs(index + 1, nums, ans);
            nums.swap(index, i);
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::permute(vec![1, 2, 3]),
        [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1]
        ]
    );
}
