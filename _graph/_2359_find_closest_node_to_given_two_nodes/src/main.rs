/*
 * @lc app=leetcode id=2359 lang=rust
 *
 * [2359] Find Closest Node to Given Two Nodes
 *
 * https://leetcode.com/problems/find-closest-node-to-given-two-nodes/description/
 *
 * algorithms
 * Medium (46.3%)
 * Likes:    2178
 * Dislikes: 626
 * Total Accepted:    64.9K
 * Total Submissions: 140.2K
 * Testcase Example:  '[[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]'
 *
 * You are given a directed graph of 'n' nodes numbered from '0' to 'n - 1', where each node has at most one outgoing edge.
 * The graph is represented with a given 0-indexed array 'edges' of size 'n', indicating that there is a directed edge from
 * node 'i' to node 'edges[i]'. If there is no outgoing edge from 'i', then 'edges[i] == -1'.
 *
 *
 * You are also given two integers 'node1' and 'node2'.
 *
 *
 * Return the index of the node that can be reached from both 'node1' and 'node2', such that the maximum between the distance from
 * 'node1' to that node, and from 'node2' to that node is minimized. If there are multiple answers, return the node with the smallest index,
 * and if no possible answer exists, return '-1'.
 *
 *
 * Note that 'edges' may contain cycles.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: edges = [2,2,3,-1], node1 = 0, node2 = 1
 * Output: 2
 * Explanation: The distance from node 0 to node 2 is 1, and the distance from node 1 to node 2 is 1.
 * The maximum of those two distances is 1. It can be proven that we cannot get a node with a smaller maximum distance than 1, so we return node 2.
 *
 *
 *
 * Example 2:
 *
 * Input: edges = [1,2,-1], node1 = 0, node2 = 2
 * Output: 2
 * Explanation: The distance from node 0 to node 2 is 2, and the distance from node 2 to itself is 0.
 * The maximum of those two distances is 2. It can be proven that we cannot get a node with a smaller maximum distance than 2, so we return node 2.
 *
 *
 *
 *
 *
 * Constraints:
 * n == edges.length
 * 2 <= n <= 10^5
 * -1 <= edges[i] < n
 * edges[i] != i
 * 0 <= node1, node2 < n
 *
 */

struct Solution {}

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let mut dist_1 = vec![i32::MAX; edges.len()];
        let mut dist_2 = vec![i32::MAX; edges.len()];

        Self::bfs(node1, &edges, &mut dist_1);
        Self::bfs(node2, &edges, &mut dist_2);

        let mut min_dist_node: i32 = -1;
        let mut min_dist = i32::MAX;

        for node in 0..edges.len() {
            if min_dist > i32::max(dist_1[node], dist_2[node]) {
                min_dist_node = node as i32;
                min_dist = i32::max(dist_1[node], dist_2[node]);
            }
        }

        min_dist_node
    }

    fn bfs(start: i32, edges: &Vec<i32>, dist: &mut Vec<i32>) {
        let mut queue = VecDeque::new();
        let mut visit = vec![false; edges.len()];

        queue.push_back(start);
        dist[start as usize] = 0;

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            if visit[node as usize] {
                continue;
            }

            visit[node as usize] = true;

            let neighbour = edges[node as usize];

            if neighbour != -1 && !visit[neighbour as usize] {
                dist[neighbour as usize] = dist[node as usize] + 1;
                queue.push_back(neighbour);
            }
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1), 2);
}
