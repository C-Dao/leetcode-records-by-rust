/*
 * @lc app=leetcode id=77 lang=rust
 *
 * [77] Combinations
 *
 * https://leetcode.com/problems/combinations/description/
 *
 * algorithms
 * Medium (64.87%)
 * Likes:    5486
 * Dislikes: 173
 * Total Accepted:    623.3K
 * Total Submissions: 938.5K
 * Testcase Example:  '4\n2'
 *
 * Given two integers n and k, return all possible combinations of k numbers
 * chosen from the range [1, n].
 *
 * You may return the answer in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 4, k = 2
 * Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
 * Explanation: There are 4 choose 2 = 6 total combinations.
 * Note that combinations are unordered, i.e., [1,2] and [2,1] are considered
 * to be the same combination.
 *
 *
 * Example 2:
 *
 *
 * Input: n = 1, k = 1
 * Output: [[1]]
 * Explanation: There is 1 choose 1 = 1 total combination.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 20
 * 1 <= k <= n
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(1, n, k, &mut vec![], &mut ans);

        ans
    }

    fn dfs(i: i32, n: i32, k: i32, cur_vec: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if cur_vec.len() == k as usize {
            ans.push(cur_vec.clone());
            return;
        };

        if i > n {
            return;
        };

        cur_vec.push(i);
        Self::dfs(i + 1, n, k, cur_vec, ans);
        cur_vec.pop();
        Self::dfs(i + 1, n, k, cur_vec, ans);
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::combine(4, 2),
        [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
    );
}
