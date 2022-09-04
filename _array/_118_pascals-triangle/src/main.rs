/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        for row in 0..num_rows as usize {
            ans.push(vec![1; row + 1]);

            for col in 1..row as usize {
                ans[row][col] = ans[row - 1][col] + ans[row - 1][col - 1];
            }
        }
        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
}
