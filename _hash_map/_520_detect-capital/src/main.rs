/*
 * @lc app=leetcode id=520 lang=rust
 *
 * [520] Detect Capital
 *
 * https://leetcode.com/problems/detect-capital/description/
 *
 * algorithms
 * Easy (55.68%)
 * Likes:    2308
 * Dislikes: 404
 * Total Accepted:    313.9K
 * Total Submissions: 557.6K
 * Testcase Example:  '"USA"'
 *
 * We define the usage of capitals in a word to be right when one of the
 * following cases holds:
 *
 *
 * All letters in this word are capitals, like "USA".
 * All letters in this word are not capitals, like "leetcode".
 * Only the first letter in this word is capital, like "Google".
 *
 *
 * Given a string word, return true if the usage of capitals in it is right.
 *
 *
 * Example 1:
 * Input: word = "USA"
 * Output: true
 * Example 2:
 * Input: word = "FlaG"
 * Output: false
 *
 *
 * Constraints:
 *
 *
 * 1 <= word.length <= 100
 * word consists of lowercase and uppercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut capital_count = 0;
        let word_bytes = word.as_bytes();

        for i in 0..word.len() {
            if word_bytes[i].is_ascii_uppercase() {
                capital_count += 1;
            }
        }

        capital_count == 0
            || capital_count == word.len()
            || (capital_count == 1 && word_bytes[0].is_ascii_uppercase())
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
    assert_eq!(Solution::detect_capital_use("leetcode".to_string()), true);
    assert_eq!(Solution::detect_capital_use("Apple".to_string()), true);
    assert_eq!(Solution::detect_capital_use("AppLe".to_string()), false);
}
