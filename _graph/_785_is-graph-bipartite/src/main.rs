/*
 * @lc app=leetcode id=785 lang=rust
 *
 * [785] Is Graph Bipartite?
 *
 * https://leetcode.com/problems/is-graph-bipartite/description/
 *
 * algorithms
 * Medium (52.10%)
 * Likes:    6140
 * Dislikes: 298
 * Total Accepted:    386.8K
 * Total Submissions: 729.9K
 * Testcase Example:  '[[1,2,3],[0,2],[0,1,3],[0,2]]'
 *
 * There is an undirected graph with n nodes, where each node is numbered
 * between 0 and n - 1. You are given a 2D array graph, where graph[u] is an
 * array of nodes that node u is adjacent to. More formally, for each v in
 * graph[u], there is an undirected edge between node u and node v. The graph
 * has the following properties:
 *
 *
 * There are no self-edges (graph[u] does not contain u).
 * There are no parallel edges (graph[u] does not contain duplicate
 * values).
 * If v is in graph[u], then u is in graph[v] (the graph is undirected).
 * The graph may not be connected, meaning there may be two nodes u and v such
 * that there is no path between them.
 *
 *
 * A graph is bipartite if the nodes can be partitioned into two independent
 * sets A and B such that every edge in the graph connects a node in set A and
 * a node in set B.
 *
 * Return true if and only if it is bipartite.
 *
 *
 * Example 1:
 *
 *
 * Input: graph = [[1,2,3],[0,2],[0,1,3],[0,2]]
 * Output: false
 * Explanation: There is no way to partition the nodes into two independent
 * sets such that every edge connects a node in one and a node in the other.
 *
 * Example 2:
 *
 *
 * Input: graph = [[1,3],[0,2],[1,3],[0,2]]
 * Output: true
 * Explanation: We can partition the nodes into two sets: {0, 2} and {1, 3}.
 *
 *
 * Constraints:
 *
 *
 * graph.length == n
 * 1 <= n <= 100
 * 0 <= graph[u].length < n
 * 0 <= graph[u][i] <= n - 1
 * graph[u] does not contain u.
 * All the values of graph[u] are unique.
 * If graph[u] contains v, then graph[v] contains u.
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    /* breadth first search */
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let size = graph.len();
        let mut colors = vec![-1; size];

        for i in 0..size {
            if colors[i] == -1 {
                if !Self::set_color(i, 0, &graph, &mut colors) {
                    return false;
                }
            }
        }
        true
    }

    fn set_color(i: usize, color: i32, graph: &Vec<Vec<i32>>, colors: &mut Vec<i32>) -> bool {
        let mut queue = VecDeque::new();

        colors[i] = color;

        queue.push_back(i);

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();

            for neighbor in graph[v].iter() {
                if colors[*neighbor as usize] != -1 {
                    if colors[*neighbor as usize] == colors[v] {
                        return false;
                    }
                } else {
                    queue.push_back(*neighbor as usize);
                    colors[*neighbor as usize] = 1 - colors[v];
                }
            }
        }

        true
    }

    /* deepth first search, iteraction */
    pub fn is_bipartite_deepth_iteraction(graph: Vec<Vec<i32>>) -> bool {
        let size = graph.len();
        let mut colors = vec![-1; size];

        for i in 0..size {
            if colors[i] == -1 {
                if !Self::set_color_deepth_iteraction(i, 0, &graph, &mut colors) {
                    return false;
                }
            }
        }
        true
    }

    fn set_color_deepth_iteraction(
        i: usize,
        color: i32,
        graph: &Vec<Vec<i32>>,
        colors: &mut Vec<i32>,
    ) -> bool {
        let mut stack = vec![];

        colors[i] = color;

        stack.push(i);

        while !stack.is_empty() {
            let v = stack.pop().unwrap();

            for neighbor in graph[v].iter() {
                if colors[*neighbor as usize] != -1 {
                    if colors[*neighbor as usize] == colors[v] {
                        return false;
                    }
                } else {
                    stack.push(*neighbor as usize);
                    colors[*neighbor as usize] = 1 - colors[v];
                }
            }
        }

        true
    }

    /* deepth first search, recursion */
    pub fn is_bipartite_deepth_recursion(graph: Vec<Vec<i32>>) -> bool {
        let size = graph.len();
        let mut colors = vec![-1; size];

        for i in 0..size {
            if colors[i] == -1 {
                if !Self::set_color_deepth_recursion(i, 0, &graph, &mut colors) {
                    return false;
                }
            }
        }
        true
    }

    fn set_color_deepth_recursion(
        i: usize,
        color: i32,
        graph: &Vec<Vec<i32>>,
        colors: &mut Vec<i32>,
    ) -> bool {
        if colors[i] != -1 {
            return colors[i] == color;
        }

        colors[i] = color;
        for neighbor in graph[i].iter() {
            if !Self::set_color_deepth_recursion(*neighbor as usize, 1 - color, graph, colors) {
                return false;
            }
        }

        true
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]),
        false
    );
    assert_eq!(
        Solution::is_bipartite_deepth_iteraction(vec![
            vec![1, 2, 3],
            vec![0, 2],
            vec![0, 1, 3],
            vec![0, 2]
        ]),
        false
    );
    assert_eq!(
        Solution::is_bipartite_deepth_recursion(vec![
            vec![1, 2, 3],
            vec![0, 2],
            vec![0, 1, 3],
            vec![0, 2]
        ]),
        false
    );
}
