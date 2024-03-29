/*
 * @lc app=leetcode id=797 lang=rust
 *
 * [797] All Paths From Source to Target
 *
 * https://leetcode.com/problems/all-paths-from-source-to-target/description/
 *
 * algorithms
 * Medium (81.11%)
 * Likes:    6268
 * Dislikes: 132
 * Total Accepted:    420.3K
 * Total Submissions: 511.2K
 * Testcase Example:  '[[1,2],[3],[3],[]]'
 *
 * Given a directed acyclic graph (DAG) of n nodes labeled from 0 to n - 1,
 * find all possible paths from node 0 to node n - 1 and return them in any
 * order.
 *
 * The graph is given as follows: graph[i] is a list of all nodes you can visit
 * from node i (i.e., there is a directed edge from node i to node
 * graph[i][j]).
 *
 *
 * Example 1:
 *
 *
 * Input: graph = [[1,2],[3],[3],[]]
 * Output: [[0,1,3],[0,2,3]]
 * Explanation: There are two paths: 0 -> 1 -> 3 and 0 -> 2 -> 3.
 *
 *
 * Example 2:
 *
 *
 * Input: graph = [[4,3,1],[3,2,4],[3],[4],[]]
 * Output: [[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
 *
 *
 *
 * Constraints:
 *
 *
 * n == graph.length
 * 2 <= n <= 15
 * 0 <= graph[i][j] < n
 * graph[i][j] != i (i.e., there will be no self-loops).
 * All the elements of graph[i] are unique.
 * The input graph is guaranteed to be a DAG.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Self::dfs(0, &graph, &mut vec![0], &mut ans);
        ans
    }

    fn dfs(cur: i32, graph: &Vec<Vec<i32>>, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if cur == graph.len() as i32 - 1 {
            ans.push(path.clone());
        } else {
            for next in graph[cur as usize].iter() {
                path.push(*next);
                Self::dfs(*next, graph, path, ans);
                path.pop();
            }
        }
    }

    pub fn all_paths_source_target_iteraction(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut stack:Vec<(i32, Vec<i32>)> = vec![(0, vec![0])];

        while let Some((cur, path)) = stack.pop() {
            if cur == graph.len() as i32 - 1 {
                ans.push(path);
            } else {
                for next in graph[cur as usize].iter() {
                    let mut new_path = path.clone();
                    new_path.push(*next);
                    stack.push((*next, new_path));
                }
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
        [[0, 1, 3], [0, 2, 3]]
    );
    assert_eq!(
        Solution::all_paths_source_target_iteraction(vec![vec![1, 2], vec![3], vec![3], vec![]]),
        [[0, 2, 3], [0, 1, 3]]
    );
}
