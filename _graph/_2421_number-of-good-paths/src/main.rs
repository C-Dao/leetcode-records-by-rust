/*
* @lc app=leetcode id=2421 lang=rust
*
* [2421] Number of Good Paths
*
* https://leetcode.com/problems/number-of-good-paths/description/
*
* algorithms
* Hard (54.6%)
* Likes:    1377
* Dislikes: 68
* Total Accepted:    29.1K
* Total Submissions: 53.3K
* Testcase Example:  '[1,3,2,1,3]\n[[0,1],[0,2],[2,3],[2,4]]'
*
*
* "
* There is a tree (i.e. a connected, undirected graph with no cycles) consisting of n nodes numbered from 0 to n - 1 and exactly n - 1 edges.
* You are given a 0-indexed integer array vals of length n where vals[i] denotes the value of the i^th node.
* You are also given a 2D integer array edges where edges[i] = [a[i], b[i]] denotes that there exists an undirected edge connecting nodes a[i] and b[i].
*
* A good path is a simple path that satisfies the following conditions:
*
* The starting node and the ending node have the same value.
* All nodes between the starting node and the ending node have values less than or equal to the starting node
* (i.e. the starting node&#39;s value should be the maximum value along the path).
*
* Return the number of distinct good paths.
*
* Note that a path and its reverse are counted as the same path. For example, 0 -> 1 is considered to be the same as 1 -> 0.
* A single node is also considered as a valid path.
*
*
*
*
* Example 1:
* Input: vals = [1,3,2,1,3], edges = [[0,1],[0,2],[2,3],[2,4]]
* Output: 6
* Explanation: There are 5 good paths consisting of a single node.
* There is 1 additional good path: 1 -> 0 -> 2 -> 4.
* (The reverse path 4 -> 2 -> 0 -> 1 is treated as the same as 1 -> 0 -> 2 -> 4.)
* Note that 0 -> 2 -> 3 is not a good path because vals[2] > vals[0].
*
*
*
* Example 2:
* Input: vals = [1,1,2,2,3], edges = [[0,1],[1,2],[2,3],[2,4]]
* Output: 7
* Explanation: There are 5 good paths consisting of a single node.
* There are 2 additional good paths: 0 -> 1 and 2 -> 3.
*
*
*
* Example 3:
* Input: vals = [1], edges = []
* Output: 1
* Explanation: The tree consists of only one node, so there is one good path.
*
*
*
* Constraints:
*
*
*
*
* n == vals.length
* 1 <= n <= 3 * 10^4
* 0 <= vals[i] <= 10^5
* edges.length == n - 1
* edges[i].length == 2
* 0 <= a[i], b[i] < n
* a[i] != b[i]
* edges represents a valid tree.
*
*
*
*
*/

struct Solution {}

// @lc code=start
use std::collections::{BTreeMap, HashMap};

struct UnionFind {
    root: Vec<i32>,
}

impl UnionFind {
    pub fn new(count: i32) -> Self {
        let mut union_find = Self {
            root: vec![0; count as usize],
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
            return true;
        }

        return false;
    }
}

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut adj = vec![vec![]; vals.len()];
        let mut values_to_nodes: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        let mut union_find = UnionFind::new(vals.len() as i32);
        let mut ans = 0;

        for edge in edges {
            adj[edge[0] as usize].push(edge[1]);
            adj[edge[1] as usize].push(edge[0]);
        }

        for (i, &val) in vals.iter().enumerate() {
            values_to_nodes.entry(val).or_insert(vec![]).push(i as i32);
        }

        for (_, nodes) in values_to_nodes {
            for &node in &nodes {
                for neighbor in &adj[node as usize] {
                    if vals[node as usize] >= vals[*neighbor as usize] {
                        union_find.union(node, *neighbor);
                    }
                }
            }

            let mut group = HashMap::new();

            for u in nodes {
                group
                    .entry(union_find.find_root(u))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            
            for count in group.values() {
                ans += count * (count + 1) / 2;
            }
        }

        ans
    }
}
// @lc code=end
fn main() {
    assert_eq!(
        Solution::number_of_good_paths(
            vec![1, 3, 2, 1, 3],
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
        ),
        6
    );
}
