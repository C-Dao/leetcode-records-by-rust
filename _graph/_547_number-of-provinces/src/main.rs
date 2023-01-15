/*
 * @lc app=leetcode id=547 lang=rust
 *
 * [547] Number of Provinces
 *
 * https://leetcode.com/problems/number-of-provinces/description/
 *
 * algorithms
 * Medium (62.78%)
 * Likes:    6853
 * Dislikes: 279
 * Total Accepted:    579.3K
 * Total Submissions: 912.3K
 * Testcase Example:  '[[1,1,0],[1,1,0],[0,0,1]]'
 *
 * There are n cities. Some of them are connected, while some are not. If city
 * a is connected directly with city b, and city b is connected directly with
 * city c, then city a is connected indirectly with city c.
 *
 * A province is a group of directly or indirectly connected cities and no
 * other cities outside of the group.
 *
 * You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the
 * i^th city and the j^th city are directly connected, and isConnected[i][j] =
 * 0 otherwise.
 *
 * Return the total number of provinces.
 *
 *
 * Example 1:
 *
 *
 * Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
 * Output: 3
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 200
 * n == isConnected.length
 * n == isConnected[i].length
 * isConnected[i][j] is 1 or 0.
 * isConnected[i][i] == 1
 * isConnected[i][j] == isConnected[j][i]
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::VecDeque;

struct UnionFind {
    root: Vec<i32>,
    count: i32,
}

impl UnionFind {
    pub fn new(count: i32) -> Self {
        let mut union_find = Self {
            root: vec![0; count as usize],
            count: count,
        };

        for i in 0..count {
            union_find.root[i as usize] = i;
        }

        union_find
    }

    pub fn find_root(&mut self, x: i32) -> i32 {
        if x != self.root[x as usize] {
            self.root[x as usize] = self.find_root(self.root[x as usize]);
        };

        self.root[x as usize]
    }

    pub fn union(&mut self, i: i32, j: i32) -> bool {
        let root_of_i = self.find_root(i);
        let root_of_j = self.find_root(j);

        if root_of_i != root_of_j {
            self.root[root_of_i as usize] = root_of_j;
            self.count -= 1;
            return true;
        }

        return false;
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

impl Solution {
    /** union find */
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut union_find = UnionFind::new(is_connected.len() as i32);

        for i in 0..is_connected.len() {
            for j in 0..is_connected.len() {
                if is_connected[i][j] == 1 {
                    union_find.union(i as i32, j as i32);
                }
            }
        }

        return union_find.count();
    }
    pub fn find_circle_num_bfs(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; is_connected.len()];

        let mut ans = 0;

        for i in 0..visited.len() {
            if !visited[i] {
                Self::find_circle_bfs(&is_connected, &mut visited, i);
                ans += 1;
            }
        }

        ans
    }

    /** bfs */
    fn find_circle_bfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize) {
        let mut queue = VecDeque::new();

        queue.push_back(i);

        while !queue.is_empty() {
            let t = queue.pop_front().unwrap();

            visited[t] = true;

            for f in 0..is_connected.len() {
                if is_connected[t][f] == 1 && !visited[f] {
                    queue.push_back(f);
                }
            }
        }
    }

    pub fn find_circle_num_dfs(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; is_connected.len()];

        let mut ans = 0;

        for i in 0..visited.len() {
            if !visited[i] {
                Self::find_circle_dfs(&is_connected, &mut visited, i);
                ans += 1;
            }
        }

        ans
    }

    /** dfs */
    fn find_circle_dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize) {
        for j in 0..is_connected.len() {
            if is_connected[i][j] == 1 && !visited[j] {
                visited[j] = true;
                Self::find_circle_dfs(is_connected, visited, j);
            }
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::find_circle_num_bfs(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::find_circle_num_dfs(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]),
        1
    );
}
