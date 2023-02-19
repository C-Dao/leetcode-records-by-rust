/*
 * @lc app=leetcode id=40 lang=rust
 *
 * [40] Combination Sum II
 *
 * https://leetcode.com/problems/combination-sum-ii/description/
 *
 * algorithms
 * Medium (52.76%)
 * Likes:    7623
 * Dislikes: 185
 * Total Accepted:    689.2K
 * Total Submissions: 1.3M
 * Testcase Example:  '[10,1,2,7,6,1,5]\n8'
 *
 * Given a collection of candidate numbers (candidates) and a target number
 * (target), find all unique combinations in candidates where the candidate
 * numbers sum to target.
 *
 * Each number in candidates may only be used once in the combination.
 *
 * Note: The solution set must not contain duplicate combinations.
 *
 *
 * Example 1:
 *
 *
 * Input: candidates = [10,1,2,7,6,1,5], target = 8
 * Output:
 * [
 * [1,1,6],
 * [1,2,5],
 * [1,7],
 * [2,6]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,5,2,1,2], target = 5
 * Output:
 * [
 * [1,2,2],
 * [5]
 * ]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= candidates.length <= 100
 * 1 <= candidates[i] <= 50
 * 1 <= target <= 30
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        candidates.sort();

        Self::dfs(0, &candidates, &mut vec![], &mut ans, target);

        ans
    }

    fn dfs(
        index: usize,
        candidates: &Vec<i32>,
        cur_set: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        target: i32,
    ) {
        if target == 0 {
            ans.push(cur_set.to_vec());
            return;
        }

        if target < 0 || index == candidates.len() {
            return;
        }

        cur_set.push(candidates[index]);

        Self::dfs(
            index + 1,
            candidates,
            cur_set,
            ans,
            target - candidates[index],
        );

        cur_set.pop();
        
        Self::dfs(
            Self::get_next_diff(candidates, index),
            candidates,
            cur_set,
            ans,
            target,
        );
    }

    fn get_next_diff(candidates: &Vec<i32>, index: usize) -> usize {
        let mut next = index;

        while next < candidates.len() && candidates[next] == candidates[index] {
            next += 1;
        }

        return next;
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
    );
}
