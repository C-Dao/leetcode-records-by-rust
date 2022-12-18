/*
 * @lc app=leetcode id=953 lang=rust
 *
 * [953] Verifying an Alien Dictionary
 *
 * https://leetcode.com/problems/verifying-an-alien-dictionary/description/
 *
 * algorithms
 * Easy (52.57%)
 * Likes:    3231
 * Dislikes: 1040
 * Total Accepted:    378.8K
 * Total Submissions: 718.4K
 * Testcase Example:  '["hello","leetcode"]\n"hlabcdefgijkmnopqrstuvwxyz"'
 *
 * In an alien language, surprisingly, they also use English lowercase letters,
 * but possibly in a different order. The order of the alphabet is some
 * permutation of lowercase letters.
 *
 * Given a sequence of words written in the alien language, and the order of
 * the alphabet, return true if and only if the given words are sorted
 * lexicographically in this alien language.
 *
 *
 * Example 1:
 *
 *
 * Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
 * Output: true
 * Explanation: As 'h' comes before 'l' in this language, then the sequence is
 * sorted.
 *
 *
 * Example 2:
 *
 *
 * Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
 * Output: false
 * Explanation: As 'd' comes after 'l' in this language, then words[0] >
 * words[1], hence the sequence is unsorted.
 *
 *
 * Example 3:
 *
 *
 * Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
 * Output: false
 * Explanation: The first three characters "app" match, and the second string
 * is shorter (in size.) According to lexicographical rules "apple" > "app",
 * because 'l' > '∅', where '∅' is defined as the blank character which is less
 * than any other character (More info).
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= words.length <= 100
 * 1 <= words[i].length <= 20
 * order.length == 26
 * All characters in words[i] and order are English lowercase letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut order_array = [0; 26];
        let order_bytes = order.as_bytes();
        for i in 0..order.len() {
            order_array[(order_bytes[i] - b'a') as usize] = i;
        }

        for i in 0..words.len() - 1 {
            if Self::is_unsorted(&words[i], &words[i + 1], &order_array) {
                return false;
            }
        }

        true
    }

    fn is_unsorted(word_a: &String, word_b: &String, order_array: &[usize; 26]) -> bool {
        let word_a_bytes = word_a.as_bytes();
        let word_b_bytes = word_b.as_bytes();
        let mut ch_index = 0;
        while ch_index < word_a.len() && ch_index < word_b.len() {
            let ch_a = word_a_bytes[ch_index];
            let ch_b = word_b_bytes[ch_index];

            if order_array[(ch_a - b'a') as usize] < order_array[(ch_b - b'a') as usize] {
                return false;
            };

            if order_array[(ch_a - b'a') as usize] > order_array[(ch_b - b'a') as usize] {
                return true;
            };

            ch_index += 1;
        }
        
        ch_index != word_a.len()
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ),
        true
    );
}
