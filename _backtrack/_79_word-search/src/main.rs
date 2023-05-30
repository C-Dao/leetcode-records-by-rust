/*
 * @lc app=leetcode id=79 lang=rust
 *
 * [79] Word Search
 *
 * https://leetcode.com/problems/word-search/description/
 *
 * algorithms
 * Medium (39.89%)
 * Likes:    13377
 * Dislikes: 545
 * Total Accepted:    1.3M
 * Total Submissions: 3.2M
 * Testcase Example:  '[["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]\n"ABCCED"'
 *
 * Given an m x n grid of characters board and a string word, return true if
 * word exists in the grid.
 *
 * The word can be constructed from letters of sequentially adjacent cells,
 * where adjacent cells are horizontally or vertically neighboring. The same
 * letter cell may not be used more than once.
 *
 *
 * Example 1:
 *
 *
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word
 * = "ABCCED"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word
 * = "SEE"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word
 * = "ABCB"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * m == board.length
 * n = board[i].length
 * 1 <= m, n <= 6
 * 1 <= word.length <= 15
 * board and word consists of only lowercase and uppercase English letters.
 *
 *
 *
 * Follow up: Could you use search pruning to make your solution faster with a
 * larger board?
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::dfs(
                    i,
                    j,
                    0,
                    &board,
                    &mut vec![vec![false; board[0].len()]; board.len()],
                    &word.chars().collect(),
                ) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        i: usize,
        j: usize,
        count: usize,
        board: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        word: &Vec<char>,
    ) -> bool {
        if board[i][j] != word[count] {
            return false;
        } else if count == word.len() - 1 {
            return true;
        }

        visited[i][j] = true;

        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        for dir in dirs {
            let next_i = i as i32 + dir.0;
            let next_j = j as i32 + dir.1;
            if next_i >= 0
                && next_i < board.len() as i32
                && next_j >= 0
                && next_j < board[0].len() as i32
                && !visited[next_i as usize][next_j as usize]
            {
                if Self::dfs(
                    next_i as usize,
                    next_j as usize,
                    count + 1,
                    board,
                    visited,
                    word,
                ) {
                    return true;
                }
                visited[next_i as usize][next_j as usize] = false;
            }
        }

        false
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ),
        true
    );
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        ),
        false
    );
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE".to_string()
        ),
        true
    );
}
