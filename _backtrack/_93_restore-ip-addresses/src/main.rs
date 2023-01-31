/*
 * @lc app=leetcode id=93 lang=rust
 *
 * [93] Restore IP Addresses
 *
 * https://leetcode.com/problems/restore-ip-addresses/description/
 *
 * algorithms
 * Medium (42.48%)
 * Likes:    3309
 * Dislikes: 668
 * Total Accepted:    335.4K
 * Total Submissions: 764.6K
 * Testcase Example:  '"25525511135"'
 *
 * A valid IP address consists of exactly four integers separated by single
 * dots. Each integer is between 0 and 255 (inclusive) and cannot have leading
 * zeros.
 *
 *
 * For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses, but
 * "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP
 * addresses.
 *
 *
 * Given a string s containing only digits, return all possible valid IP
 * addresses that can be formed by inserting dots into s. You are not allowed
 * to reorder or remove any digits in s. You may return the valid IP addresses
 * in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "25525511135"
 * Output: ["255.255.11.135","255.255.111.35"]
 *
 *
 * Example 2:
 *
 *
 * Input: s = "0000"
 * Output: ["0.0.0.0"]
 *
 *
 * Example 3:
 *
 *
 * Input: s = "101023"
 * Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 20
 * s consists of digits only.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ans = vec![];

        Self::dfs(0, 0, s.as_bytes(), &mut vec![0; 4], &mut ans);

        ans
    }

    fn dfs(
        cur: usize,
        segment_count: usize,
        s: &[u8],
        segments: &mut Vec<i32>,
        ans: &mut Vec<String>,
    ) {
        if segment_count == 4 && cur == s.len() {
            ans.push(
                segments
                    .iter()
                    .map(|str| str.to_string())
                    .collect::<Vec<String>>()
                    .join("."),
            );
            return;
        };

        if segment_count == 4 || cur == s.len() {
            return;
        };

        if s[cur] == b'0' {
            segments[segment_count] = 0;
            Self::dfs(cur + 1, segment_count + 1, s, segments, ans);
        }

        let mut addr = 0;

        for i in cur..s.len() {
            addr = addr * 10 + s[i] as i32 - b'0' as i32;

            if addr > 0 && addr < 256 {
                segments[segment_count] = addr;
                Self::dfs(i + 1, segment_count + 1, s, segments, ans);
            } else {
                break;
            }
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::restore_ip_addresses("101023".to_string()),
        [
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3"
        ]
    );
}
