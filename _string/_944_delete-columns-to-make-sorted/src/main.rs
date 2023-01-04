/*
 * @lc app=leetcode id=944 lang=rust
 *
 * [944] Delete Columns to Make Sorted
 *
 * https://leetcode.com/problems/delete-columns-to-make-sorted/description/
 *
 * algorithms
 * Easy (69.87%)
 * Likes:    1267
 * Dislikes: 2618
 * Total Accepted:    135.7K
 * Total Submissions: 182.6K
 * Testcase Example:  '["cba","daf","ghi"]'
 *
 * You are given an array of n strings strs, all of the same length.
 *
 * The strings can be arranged such that there is one on each line, making a
 * grid. For example, strs = ["abc", "bce", "cae"] can be arranged as:
 *
 *
 * abc
 * bce
 * cae
 *
 *
 * You want to delete the columns that are not sorted lexicographically. In the
 * above example (0-indexed), columns 0 ('a', 'b', 'c') and 2 ('c', 'e', 'e')
 * are sorted while column 1 ('b', 'c', 'a') is not, so you would delete column
 * 1.
 *
 * Return the number of columns that you will delete.
 *
 *
 * Example 1:
 *
 *
 * Input: strs = ["cba","daf","ghi"]
 * Output: 1
 * Explanation: The grid looks as follows:
 * ⁠ cba
 * ⁠ daf
 * ⁠ ghi
 * Columns 0 and 2 are sorted, but column 1 is not, so you only need to delete
 * 1 column.
 *
 *
 * Example 2:
 *
 *
 * Input: strs = ["a","b"]
 * Output: 0
 * Explanation: The grid looks as follows:
 * ⁠ a
 * ⁠ b
 * Column 0 is the only column and is sorted, so you will not delete any
 * columns.
 *
 *
 * Example 3:
 *
 *
 * Input: strs = ["zyx","wvu","tsr"]
 * Output: 3
 * Explanation: The grid looks as follows:
 * ⁠ zyx
 * ⁠ wvu
 * ⁠ tsr
 * All 3 columns are not sorted, so you will delete all 3.
 *
 *
 *
 * Constraints:
 *
 *
 * n == strs.length
 * 1 <= n <= 100
 * 1 <= strs[i].length <= 1000
 * strs[i] consists of lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs_bytes = strs
            .iter()
            .map(|str| str.as_bytes())
            .collect::<Vec<&[u8]>>();

        let mut ans = 0;
        
        for i in 0..strs[0].len() {
            for k in 1..strs_bytes.len() {
                if strs_bytes[k][i] < strs_bytes[k - 1][i] {
                    ans += 1;
                    break;
                }
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::min_deletion_size(vec![
            "cba".to_string(),
            "daf".to_string(),
            "ghi".to_string()
        ]),
        1
    );
}
