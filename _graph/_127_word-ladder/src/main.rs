/*
 * @lc app=leetcode id=127 lang=rust
 *
 * [127] Word Ladder
 *
 * https://leetcode.com/problems/word-ladder/description/
 *
 * algorithms
 * Hard (35.88%)
 * Likes:    9925
 * Dislikes: 1769
 * Total Accepted:    888.1K
 * Total Submissions: 2.4M
 * Testcase Example:  '"hit"\n"cog"\n["hot","dot","dog","lot","log","cog"]'
 *
 * A transformation sequence from word beginWord to word endWord using a
 * dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... ->
 * sk such that:
 *
 *
 * Every adjacent pair of words differs by a single letter.
 * Every si for 1 <= i <= k is in wordList. Note that beginWord does not need
 * to be in wordList.
 * sk == endWord
 *
 *
 * Given two words, beginWord and endWord, and a dictionary wordList, return
 * the number of words in the shortest transformation sequence from beginWord
 * to endWord, or 0 if no such sequence exists.
 *
 *
 * Example 1:
 *
 *
 * Input: beginWord = "hit", endWord = "cog", wordList =
 * ["hot","dot","dog","lot","log","cog"]
 * Output: 5
 * Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot"
 * -> "dog" -> cog", which is 5 words long.
 *
 *
 * Example 2:
 *
 *
 * Input: beginWord = "hit", endWord = "cog", wordList =
 * ["hot","dot","dog","lot","log"]
 * Output: 0
 * Explanation: The endWord "cog" is not in wordList, therefore there is no
 * valid transformation sequence.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= beginWord.length <= 10
 * endWord.length == beginWord.length
 * 1 <= wordList.length <= 5000
 * wordList[i].length == beginWord.length
 * beginWord, endWord, and wordList[i] consist of lowercase English
 * letters.
 * beginWord != endWord
 * All the words in wordList are unique.
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut words = HashSet::new();
        let mut len = 1;
        let characters = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();

        for word in word_list {
            words.insert(word);
        }

        if !words.contains(&end_word) {
            return 0;
        }

        queue.push_back(begin_word.clone());

        while queue.len() > 0 {
            let queue_size = queue.len();

            for _ in 0..queue_size {
                let word = queue.pop_front().unwrap();

                for j in 0..word.len() {
                    for k in &characters {
                        let new_word =
                            (&word[0..j]).to_string() + &(k.to_string()) + &word[j + 1..];

                        if new_word == end_word {
                            return len + 1;
                        }

                        if words.contains(&new_word) && !visited.contains(&new_word) {
                            queue.push_back(new_word.clone());
                            visited.insert(new_word);
                        }
                    }
                }
            }

            len += 1;
        }

        0
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::ladder_length(
            format!("hit"),
            format!("cog"),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string()
            ]
        ),
        5
    );
}
