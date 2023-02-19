/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 *
 * https://leetcode.com/problems/combination-sum/description/
 *
 * algorithms
 * Medium (66.38%)
 * Likes:    14593
 * Dislikes: 291
 * Total Accepted:    1.4M
 * Total Submissions: 2M
 * Testcase Example:  '[2,3,6,7]\n7'
 *
 * Given an array of distinct integers candidates and a target integer target,
 * return a list of all unique combinations of candidates where the chosen
 * numbers sum to target. You may return the combinations in any order.
 *
 * The same number may be chosen from candidates an unlimited number of times.
 * Two combinations are unique if the frequency of at least one of the chosen
 * numbers is different.
 *
 * The test cases are generated such that the number of unique combinations
 * that sum up to target is less than 150 combinations for the given input.
 *
 *
 * Example 1:
 *
 *
 * Input: candidates = [2,3,6,7], target = 7
 * Output: [[2,2,3],[7]]
 * Explanation:
 * 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple
 * times.
 * 7 is a candidate, and 7 = 7.
 * These are the only two combinations.
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,3,5], target = 8
 * Output: [[2,2,2,2],[2,3,3],[3,5]]
 *
 *
 * Example 3:
 *
 *
 * Input: candidates = [2], target = 1
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= candidates.length <= 30
 * 2 <= candidates[i] <= 40
 * All elements of candidates are distinct.
 * 1 <= target <= 40
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(0, &mut vec![], target, &candidates, &mut ans);

        ans
    }

    pub fn dfs(
        index: usize,
        cur_set: &mut Vec<i32>,
        target: i32,
        candidates: &Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if index == candidates.len() || target < 0 {
            return;
        }

        if target == 0 {
            ans.push(cur_set.clone());
            return;
        };

        cur_set.push(candidates[index]);
        Self::dfs(index, cur_set, target - candidates[index], candidates, ans);
        cur_set.pop();
        Self::dfs(index + 1, cur_set, target, candidates, ans);
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
}
