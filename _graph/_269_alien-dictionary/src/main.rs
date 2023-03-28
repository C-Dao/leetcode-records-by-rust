/*
 * @lc app=leetcode id=269 lang=rust
 *
 * [269] Alien Dictionary
 *
 * https://leetcode.com/problems/alien-dictionary/description/
 *
 * algorithms
 * Medium (47.12%)
 * Likes:    8719
 * Dislikes: 289
 * Total Accepted:    785K
 * Total Submissions: 1.6M
 * Testcase Example:  '["wrt","wrf","er","ett","rftt"]\n"wertf"'
 *
 * There is an alien language that uses the English alphabet, and this language has a different alphabetic
 * order than English.
 *
 * Given a list of strings words, which is a dictionary for the language, the strings in words have been
 * sorted in the alphabetical order of the new language.
 *
 * You are asked to restore the known alphabetical order of the language according to the dictionary and
 * sort them in increasing alphabetical order. If no legal alphabetical order exists, return "" . If there
 * are multiple possible legal alphabetic orders, return any one of them.
 *
 * The dictionary order of the string s is less than the string t. There are two cases:
 *
 * At the first different letter, if the letter in s precedes the letter in t in the alphabetical order of
 * this alien language, then the dictionary order of s is less than t.
 *
 * If the preceding min(s.length, t.length) letters are the same, then the dictionary order of s is also
 * less than t if s.length < t.length.
 *
 * Example 1:
 *
 *
 * Input: words = ["wrt","wrf","er","ett","rftt"]
 * Output: "wertf"
 *
 *
 * Example 2:
 *
 *
 * Input: words = ["z","x"]
 * Output: "zx"
 *
 *
 * Example 3:
 *
 *
 * Input: words = ["z","x","z"]
 * Output: ""
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= words.length <= 100
 * 1 <= words[i].length <= 100
 * words[i] consists of lowercase letters only
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph = HashMap::new();
        let mut indegrees = HashMap::new();

        for word in words.iter() {
            for &ch in word.as_bytes() {
                if !graph.contains_key(&ch) {
                    graph.insert(ch, HashSet::new());
                }

                if !indegrees.contains_key(&ch) {
                    indegrees.insert(ch, 0);
                }
            }
        }

        for i in 1..words.len() {
            let w1 = words[i - 1].clone();
            let w2 = words[i].clone();

            if w1.starts_with(&w2) && w1 != w2 {
                return format!("");
            }

            let mut j = 0;

            while j < w1.len() && j < w2.len() {
                let ch1 = w1.as_bytes()[j];
                let ch2 = w2.as_bytes()[j];

                if ch1 != ch2 {
                    if !graph.get(&ch1).unwrap().contains(&ch2) {
                        graph.get_mut(&ch1).unwrap().insert(ch2);
                        indegrees.insert(ch2, indegrees.get(&ch2).unwrap() + 1);
                    }
                    break;
                }
                j += 1;
            }
        }

        let mut queue = VecDeque::new();
        let mut ans = vec![];
        for ch in indegrees.keys() {
            if indegrees.get(ch).unwrap() == &0 {
                queue.push_back(*ch);
            }
        }

        while !queue.is_empty() {
            let ch = queue.pop_front().unwrap();
            ans.push(ch);

            for &next in graph.get(&ch).unwrap() {
                indegrees.insert(next, indegrees.get(&next).unwrap() - 1);
                if indegrees.get(&next).unwrap() == &0 {
                    queue.push_back(next);
                }
            }
        }

        if ans.len() == graph.len() {
            String::from_utf8(ans).unwrap()
        } else {
            format!("")
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::alien_order(vec![
            "wrt".to_string(),
            "wrf".to_string(),
            "er".to_string(),
            "ett".to_string(),
            "rftt".to_string(),
        ]),
        "wertf"
    );
}
