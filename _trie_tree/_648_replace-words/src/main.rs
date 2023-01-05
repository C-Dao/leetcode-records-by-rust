/*
 * @lc app=leetcode id=648 lang=rust
 *
 * [648] Replace Words
 *
 * https://leetcode.com/problems/replace-words/description/
 *
 * algorithms
 * Medium (62.42%)
 * Likes:    1845
 * Dislikes: 162
 * Total Accepted:    112.7K
 * Total Submissions: 179.7K
 * Testcase Example:  '["cat","bat","rat"]\n"the cattle was rattled by the battery"'
 *
 * In English, we have a concept called root, which can be followed by some
 * other word to form another longer word - let's call this word successor. For
 * example, when the root "an" is followed by the successor word "other", we
 * can form a new word "another".
 *
 * Given a dictionary consisting of many roots and a sentence consisting of
 * words separated by spaces, replace all the successors in the sentence with
 * the root forming it. If a successor can be replaced by more than one root,
 * replace it with the root that has the shortest length.
 *
 * Return the sentence after the replacement.
 *
 *
 * Example 1:
 *
 *
 * Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled
 * by the battery"
 * Output: "the cat was rat by the bat"
 *
 *
 * Example 2:
 *
 *
 * Input: dictionary = ["a","b","c"], sentence = "aadsfasf absbs bbab cadsfafs"
 * Output: "a a b c"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= dictionary.length <= 1000
 * 1 <= dictionary[i].length <= 100
 * dictionary[i] consists of only lower-case letters.
 * 1 <= sentence.length <= 10^6
 * sentence consists of only lower-case letters and spaces.
 * The number of words in sentence is in the range [1, 1000]
 * The length of each word in sentence is in the range [1, 1000]
 * Every two consecutive words in sentence will be separated by exactly one
 * space.
 * sentence does not have leading or trailing spaces.
 *
 *
 */

struct Solution {}
// @lc code=start

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    prefix_cache: Option<String>,
}

impl Trie {
    pub fn build_trie(dictionary: Vec<String>) -> Trie {
        let mut root = Trie::default();
        for prefix in dictionary {
            let mut node = &mut root;
            for u in prefix.as_bytes() {
                let index = (*u - b'a') as usize;
                node = node.children[index].get_or_insert_with(|| Box::new(Trie::default()));
            }
            node.prefix_cache = Some(prefix);
        }

        root
    }

    pub fn find_prefix(&self, word: String) -> Option<String> {
        let mut node = self;

        for u in word.as_bytes() {
            let index = (*u - b'a') as usize;
            let child = node.children[index].as_ref();

            if child.is_none() {
                return Some(word);
            }

            if child.is_some() && child.unwrap().prefix_cache.is_some() {
                return child.unwrap().prefix_cache.clone();
            }

            node = child.unwrap();
        }

        Some(word)
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let trie = Trie::build_trie(dictionary);
        let words = sentence.split_whitespace();

        words
            .into_iter()
            .filter_map(|word| trie.find_prefix(word.into()))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::replace_words(
            vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
            "the cattle was rattled by the battery".to_string()
        ),
        "the cat was rat by the bat"
    );
}
