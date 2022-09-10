/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 *
 * https://leetcode.com/problems/implement-strstr/description/
 *
 * algorithms
 * Easy (36.55%)
 * Likes:    4916
 * Dislikes: 3902
 * Total Accepted:    1.4M
 * Total Submissions: 3.8M
 * Testcase Example:  '"hello"\n"ll"'
 *
 * Implement strStr().
 * 
 * Given two strings needle and haystack, return the index of the first
 * occurrence of needle in haystack, or -1 if needle is not part of haystack.
 * 
 * Clarification:
 * 
 * What should we return when needle is an empty string? This is a great
 * question to ask during an interview.
 * 
 * For the purpose of this problem, we will return 0 when needle is an empty
 * string. This is consistent to C's strstr() and Java's indexOf().
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= haystack.length, needle.length <= 10^4
 * haystack and needle consist of only lowercase English characters.
 * 
 * 
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.chars().count() == 0 {
            return 0;
        }
        let next_vec = Self::kmp_next_vec(&needle);

        let (mut i, mut j) = (0, 0);
        while i < haystack.chars().count() {
            while j > 0 && haystack.chars().nth(i).unwrap() != needle.chars().nth(j).unwrap() {
                j = next_vec[j - 1];
            }
            if haystack.chars().nth(i).unwrap() == needle.chars().nth(j).unwrap() {
                j += 1;
            }
            if j == needle.chars().count() {
                return i as i32 - needle.chars().count() as i32 + 1;
            }
            i += 1;
        }
        return -1;
    }

    fn kmp_next_vec(sub: &String) -> Vec<usize> {
        let mut next = vec![0; sub.chars().count()];
        for i in 1..sub.chars().count() {
            let mut t = next[i - 1];
            while t > 0 && sub.chars().nth(i).unwrap() != sub.chars().nth(t).unwrap() {
                t = next[t - 1];
            }
            if sub.chars().nth(i).unwrap() == sub.chars().nth(t).unwrap() {
                t += 1;
            }

            next[i] = t;
        }
        next
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
}
