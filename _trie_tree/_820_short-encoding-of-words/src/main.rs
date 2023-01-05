/*
 * @lc app=leetcode id=820 lang=rust
 *
 * [820] Short Encoding of Words
 *
 * https://leetcode.com/problems/short-encoding-of-words/description/
 *
 * algorithms
 * Medium (60.68%)
 * Likes:    1639
 * Dislikes: 629
 * Total Accepted:    90.7K
 * Total Submissions: 149.5K
 * Testcase Example:  '["time","me","bell"]'
 *
 * A valid encoding of an array of words is any reference string s and array of
 * indices indices such that:
 *
 *
 * words.length == indices.length
 * The reference string s ends with the '#' character.
 * For each index indices[i], the substring of s starting from indices[i] and
 * up to (but not including) the next '#' character is equal to words[i].
 *
 *
 * Given an array of words, return the length of the shortest reference string
 * s possible of any valid encoding of words.
 *
 *
 * Example 1:
 *
 *
 * Input: words = ["time", "me", "bell"]
 * Output: 10
 * Explanation: A valid encoding would be s = "time#bell#" and indices = [0, 2,
 * 5].
 * words[0] = "time", the substring of s starting from indices[0] = 0 to the
 * next '#' is underlined in "time#bell#"
 * words[1] = "me", the substring of s starting from indices[1] = 2 to the next
 * '#' is underlined in "time#bell#"
 * words[2] = "bell", the substring of s starting from indices[2] = 5 to the
 * next '#' is underlined in "time#bell#"
 *
 *
 * Example 2:
 *
 *
 * Input: words = ["t"]
 * Output: 2
 * Explanation: A valid encoding would be s = "t#" and indices = [0].
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= words.length <= 2000
 * 1 <= words[i].length <= 7
 * words[i] consists of only lowercase letters.
 *
 *
 */

struct Solution {}

// @lc code=start

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut s = self;

        for u in word.as_bytes().into_iter().rev() {
            s = s.children[*u as usize - 'a' as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }
    }

    pub fn dfs(&self, length: i32, total: &mut i32) {
        let mut is_leaf = true;

        for i in 0..26 {
            if let Some(ref child) = self.children[i] {
                is_leaf = false;
                child.dfs(length + 1, total);
            }
        }
        if is_leaf {
            *total += length;
        }
    }
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut root = Trie::new();
        let mut total = 0;

        words.into_iter().for_each(|word| root.insert(word));

        root.dfs(1, &mut total);

        total
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::minimum_length_encoding(vec![
            "time".to_string(),
            "me".to_string(),
            "bell".to_string()
        ]),
        10
    );
}
