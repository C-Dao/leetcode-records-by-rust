/*
 * @lc app=leetcode id=131 lang=rust
 *
 * [131] Palindrome Partitioning
 *
 * https://leetcode.com/problems/palindrome-partitioning/description/
 *
 * algorithms
 * Medium (60.69%)
 * Likes:    8992
 * Dislikes: 286
 * Total Accepted:    553.7K
 * Total Submissions: 881K
 * Testcase Example:  '"aab"'
 *
 * Given a string s, partition s such that every substring of the partition is
 * a palindrome. Return all possible palindrome partitioning of s.
 *
 *
 * Example 1:
 * Input: s = "aab"
 * Output: [["a","a","b"],["aa","b"]]
 * Example 2:
 * Input: s = "a"
 * Output: [["a"]]
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 16
 * s contains only lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut ans = vec![];
        Self::dfs(0, &mut ans, &mut vec![], &s.as_bytes());

        ans
    }

    fn dfs(i: usize, ans: &mut Vec<Vec<String>>, cur_palindrome: &mut Vec<String>, s: &[u8]) {
        if i == s.len() {
            ans.push(cur_palindrome.clone());
        }
        for j in (i + 1)..=s.len() {
            if Self::is_palindrome(&s[i..j]) {
                cur_palindrome.push(String::from_utf8(s[i..j].to_vec()).unwrap());
                Self::dfs(j, ans, cur_palindrome, s);
                cur_palindrome.pop();
            }
        }
    }

    fn is_palindrome(s: &[u8]) -> bool {
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::partition("aab".to_string()),
        [vec!["a", "a", "b"], vec!["aa", "b"]]
    );
}
