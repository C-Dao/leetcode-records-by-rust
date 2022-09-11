/*
 * @lc app=leetcode id=409 lang=rust
 *
 * [409] Longest Palindrome
 *
 * https://leetcode.com/problems/longest-palindrome/description/
 *
 * algorithms
 * Easy (54.09%)
 * Likes:    3419
 * Dislikes: 198
 * Total Accepted:    355.7K
 * Total Submissions: 653.8K
 * Testcase Example:  '"abccccdd"'
 *
 * Given a string s which consists of lowercase or uppercase letters, return
 * the length of the longest palindrome that can be built with those letters.
 *
 * Letters are case sensitive, for example, "Aa" is not considered a palindrome
 * here.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "abccccdd"
 * Output: 7
 * Explanation: One longest palindrome that can be built is "dccaccd", whose
 * length is 7.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "a"
 * Output: 1
 * Explanation: The longest palindrome that can be built is "a", whose length
 * is 1.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 2000
 * s consists of lowercase and/or uppercase English letters only.
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count_map: HashMap<char, i32> = HashMap::new();
        let mut ans = 0;

        for char in s.chars() {
            let count = match count_map.get_mut(&char) {
                Some(count) => *count + 1,
                None => 1,
            };

            count_map.insert(char, count);
        }

        for (_, count) in count_map.iter() {
            ans += count / 2 * 2;
            if count % 2 == 1 && ans % 2 == 0 {
                ans += 1;
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
}
