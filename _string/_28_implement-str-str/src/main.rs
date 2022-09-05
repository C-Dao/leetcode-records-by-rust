/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
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
