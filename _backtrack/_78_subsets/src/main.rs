/*
 * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 *
 * https://leetcode.com/problems/subsets/description/
 *
 * algorithms
 * Medium (72.40%)
 * Likes:    13072
 * Dislikes: 185
 * Total Accepted:    1.3M
 * Total Submissions: 1.8M
 * Testcase Example:  '[1,2,3]'
 *
 * Given an integer array nums of unique elements, return all possible subsets
 * (the power set).
 *
 * The solution set must not contain duplicate subsets. Return the solution in
 * any order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3]
 * Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0]
 * Output: [[],[0]]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10
 * -10 <= nums[i] <= 10
 * All the numbers of nums are unique.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(0, &mut vec![], &nums, &mut ans);

        ans
    }

    pub fn dfs(index: usize, cur_set: &mut Vec<i32>, nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            ans.push(cur_set.clone());
            return;
        };

        cur_set.push(nums[index]);
        Self::dfs(index + 1, cur_set, nums, ans);
        cur_set.pop();
        Self::dfs(index + 1, cur_set, nums, ans);
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::subsets(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ]
    );
}
