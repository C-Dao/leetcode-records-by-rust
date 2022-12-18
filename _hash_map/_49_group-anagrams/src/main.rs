/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 *
 * https://leetcode.com/problems/group-anagrams/description/
 *
 * algorithms
 * Medium (64.88%)
 * Likes:    13540
 * Dislikes: 400
 * Total Accepted:    1.8M
 * Total Submissions: 2.7M
 * Testcase Example:  '["eat","tea","tan","ate","nat","bat"]'
 *
 * Given an array of strings strs, group the anagrams together. You can return
 * the answer in any order.
 *
 * An Anagram is a word or phrase formed by rearranging the letters of a
 * different word or phrase, typically using all the original letters exactly
 * once.
 *
 *
 * Example 1:
 * Input: strs = ["eat","tea","tan","ate","nat","bat"]
 * Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 * Example 2:
 * Input: strs = [""]
 * Output: [[""]]
 * Example 3:
 * Input: strs = ["a"]
 * Output: [["a"]]
 *
 *
 * Constraints:
 *
 *
 * 1 <= strs.length <= 10^4
 * 0 <= strs[i].length <= 100
 * strs[i] consists of lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u32; 26], Vec<String>> = HashMap::new();

        for str in strs {
            let mut letter_counts = [0; 26];
            let str_bytes = str.as_bytes();
            for i in 0..str.len() {
                letter_counts[(str_bytes[i] - b'a') as usize] += 1;
            }

            groups.entry(letter_counts).or_default().push(str.clone());
        }

        groups.into_values().collect()
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        {
            let mut groups = Solution::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string(),
            ]);
            groups.sort_by(|a, b| a[0].cmp(&b[0]));
            groups
        },
        vec![
            vec!["bat".to_string()],
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
        ]
    );

    assert_eq!(
        {
            let mut groups = Solution::group_anagrams(vec![
                "abbbbbbbbbbb".to_string(),
                "aaaaaaaaaaab".to_string(),
            ]);
            groups.sort_by(|a, b| a[0].cmp(&b[0]));
            groups
        },
        vec![
            vec!["aaaaaaaaaaab".to_string()],
            vec!["abbbbbbbbbbb".to_string()]
        ]
    );
}
