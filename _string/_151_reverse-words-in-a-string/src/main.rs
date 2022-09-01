/*
 * @lc app=leetcode id=151 lang=rust
 *
 * [151] Reverse Words in a String
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut split_s = s.split_whitespace().collect::<Vec<&str>>();
        split_s.reverse();
        split_s.join(" ")
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::reverse_words("Hello, world!".to_string()),
        "world! Hello,"
    );
}
