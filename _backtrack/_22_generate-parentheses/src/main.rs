/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 *
 * https://leetcode.com/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (70.80%)
 * Likes:    16457
 * Dislikes: 641
 * Total Accepted:    1.3M
 * Total Submissions: 1.8M
 * Testcase Example:  '3'
 *
 * Given n pairs of parentheses, write a function to generate all combinations
 * of well-formed parentheses.
 *
 *
 * Example 1:
 * Input: n = 3
 * Output: ["((()))","(()())","(())()","()(())","()()()"]
 * Example 2:
 * Input: n = 1
 * Output: ["()"]
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 8
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = vec![];
        Self::dfs(n, n, &mut vec![], &mut ans);
        ans
    }

    fn dfs(left: i32, right: i32, cur_set: &mut Vec<u8>, ans: &mut Vec<String>) {
        if left == 0 && right == 0 {
            ans.push(String::from_utf8(cur_set.clone()).unwrap());
            return;
        };

        if left > 0 {
            cur_set.push('(' as u8);
            Self::dfs(left - 1, right, cur_set, ans);
            cur_set.pop();
        }

        if left < right {
            cur_set.push(')' as u8);
            Self::dfs(left, right - 1, cur_set, ans);
            cur_set.pop();
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        ["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
}
