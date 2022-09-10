/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 *
 * https://leetcode.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (39.96%)
 * Likes:    9929
 * Dislikes: 3322
 * Total Accepted:    1.8M
 * Total Submissions: 4.5M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * Write a function to find the longest common prefix string amongst an array
 * of strings.
 * 
 * If there is no common prefix, return an empty string "".
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] consists of only lowercase English letters.
 * 
 * 
 */


struct Solution {}

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }
        let mut min_str = strs.iter().max().unwrap().to_string();
        for str in strs {
            while !str.starts_with(&min_str) {
                min_str = min_str.get(0..min_str.len() - 1).unwrap().to_string();
            }
        }
        min_str
    }

    pub fn longest_common_prefix_another_edition(strs: Vec<String>) -> String {
        strs.iter()
            .max()
            .unwrap()
            .chars()
            .zip(strs.iter().min().unwrap().chars())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect()
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::longest_common_prefix_another_edition(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ]),
        Solution::longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ])
    );
}
