/*
 * @lc app=leetcode id=498 lang=rust
 *
 * [498] Diagonal Traverse
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let row_len: i32 = mat.len() as i32;
        let col_len: i32 = mat[0].len() as i32;

        let mut result: Vec<i32> = Vec::new();
        for i in 0..(row_len + col_len - 1) {
            if i % 2 == 0 {
                let mut x: i32 = if i < row_len { i } else { row_len - 1 };
                let mut y: i32 = if i < row_len { 0 } else { i - row_len + 1 };
                while x >= 0 && y < col_len {
                    result.push(mat[x as usize][y as usize]);
                    x -= 1;
                    y += 1;
                }
            } else {
                let mut x: i32 = if i < col_len { 0 } else { i - col_len + 1 };
                let mut y: i32 = if i < col_len { i } else { col_len - 1 };
                while x < row_len  && y >= 0 {
                    result.push(mat[x as usize][y as usize]);
                    x += 1;
                    y -= 1;
                }
            }
        }
        result
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
    );
    assert_eq!(
        Solution::find_diagonal_order(vec![vec![2,3]]),
        vec![2, 3]
    );
}
