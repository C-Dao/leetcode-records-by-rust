/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for i in 0..len / 2 {
            for j in 0..(len + 1) / 2 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[len - j - 1][i];
                matrix[len - j - 1][i] = matrix[len - i - 1][len - j - 1];
                matrix[len - i - 1][len - j - 1] = matrix[j][len - i - 1];
                matrix[j][len - i - 1] = temp;
            }
        }
    }
}
// @lc code=end

fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let rotate_matrix = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
    Solution::rotate(&mut matrix);
    assert_eq!(rotate_matrix, matrix);
}
