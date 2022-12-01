/*
 * @lc app=leetcode id=318 lang=rust
 *
 * [318] Maximum Product of Word Lengths
 *
 * https://leetcode.com/problems/maximum-product-of-word-lengths/description/
 *
 * algorithms
 * Medium (60.22%)
 * Likes:    3120
 * Dislikes: 123
 * Total Accepted:    197K
 * Total Submissions: 328.1K
 * Testcase Example:  '["abcw","baz","foo","bar","xtfn","abcdef"]'
 *
 * Given a string array words, return the maximum value of length(word[i]) *
 * length(word[j]) where the two words do not share common letters. If no such
 * two words exist, return 0.
 *
 *
 * Example 1:
 *
 *
 * Input: words = ["abcw","baz","foo","bar","xtfn","abcdef"]
 * Output: 16
 * Explanation: The two words can be "abcw", "xtfn".
 *
 *
 * Example 2:
 *
 *
 * Input: words = ["a","ab","abc","d","cd","bcd","abcd"]
 * Output: 4
 * Explanation: The two words can be "ab", "cd".
 *
 *
 * Example 3:
 *
 *
 * Input: words = ["a","aa","aaa","aaaa"]
 * Output: 0
 * Explanation: No such pair of words.
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= words.length <= 1000
 * 1 <= words[i].length <= 1000
 * words[i] consists only of lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut flags = vec![0; words.len()];
        let mut ans = 0;

        for i in 0..words.len() {
            for ch in words[i].as_bytes() {
                flags[i] |= 1 << (ch - 'a' as u8);
            }
        }

        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                if (flags[i] & flags[j]) == 0 {
                    ans = i32::max(ans, (words[i].len() * words[j].len()) as i32);
                }
            }
        }

        ans
    }

    pub fn max_product_2(words: Vec<String>) -> i32 {
        let mut flags = vec![vec![false; 26]; words.len()];
        let mut ans = 0;

        for i in 0..words.len() {
            for ch in words[i].as_bytes() {
                flags[i][(ch - 'a' as u8) as usize] = true;
            }
        }

        for i in 0..words.len() {
            for j in  (i + 1)..words.len() {
                for k in 0..26 {
                    if flags[i][k] && flags[j][k] {
                        break;
                    }
                    if k == 25 {
                        ans = i32::max(ans, (words[i].len() * words[j].len()) as i32);
                    }
                }
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::max_product(vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string()
        ]),
        16
    );
    assert_eq!(
        Solution::max_product_2(vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string()
        ]),
        16
    );
}
