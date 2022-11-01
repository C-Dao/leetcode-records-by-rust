/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 *
 * https://leetcode.com/problems/zigzag-conversion/description/
 *
 * algorithms
 * Medium (42.11%)
 * Likes:    4371
 * Dislikes: 9668
 * Total Accepted:    849.3K
 * Total Submissions: 2M
 * Testcase Example:  '"PAYPALISHIRING"\n3'
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number
 * of rows like this: (you may want to display this pattern in a fixed font for
 * better legibility)
 *
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 *
 * Write the code that will take a string and make this conversion given a
 * number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 *
 * Example 3:
 *
 *
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 1 <= numRows <= 1000
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let len: i32 = s.len() as i32;

        if num_rows <= 1 || num_rows >= len {
            return s;
        }

        let mut ans = vec![];
        let s_bytes = s.as_bytes();
        let step = 2 * num_rows - 2;

        for i in 0..num_rows {
            let mut j = i;
            let mut add = i * 2;
            while j < len {
                ans.push(s_bytes[j as usize]);
                add = step - add;
                j += if i == 0 || i == num_rows - 1 {
                    step
                } else {
                    add
                };
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
}
