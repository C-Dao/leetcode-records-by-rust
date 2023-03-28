/*
 * @lc app=leetcode id=684 lang=rust
 *
 * [684] Redundant Connection
 *
 * https://leetcode.com/problems/redundant-connection/description/
 *
 * algorithms
 * Medium (61.56%)
 * Likes:    4994
 * Dislikes: 342
 * Total Accepted:    261.8K
 * Total Submissions: 421.1K
 * Testcase Example:  '[[1,2],[1,3],[2,3]]'
 *
 * In this problem, a tree is an undirected graph that is connected and has no
 * cycles.
 *
 * You are given a graph that started as a tree with n nodes labeled from 1 to
 * n, with one additional edge added. The added edge has two different vertices
 * chosen from 1 to n, and was not an edge that already existed. The graph is
 * represented as an array edges of length n where edges[i] = [ai, bi]
 * indicates that there is an edge between nodes ai and bi in the graph.
 *
 * Return an edge that can be removed so that the resulting graph is a tree of
 * n nodes. If there are multiple answers, return the answer that occurs last
 * in the input.
 *
 *
 * Example 1:
 *
 *
 * Input: edges = [[1,2],[1,3],[2,3]]
 * Output: [2,3]
 *
 *
 * Example 2:
 *
 *
 * Input: edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
 * Output: [1,4]
 *
 *
 *
 * Constraints:
 *
 *
 * n == edges.length
 * 3 <= n <= 1000
 * edges[i].length == 2
 * 1 <= ai < bi <= edges.length
 * ai != bi
 * There are no repeated edges.
 * The given graph is connected.
 *
 *
 */

struct Solution {}

// @lc code=start

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
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new((edges.len() + 1) as i32);

        for edge in edges {
            if uf.find_root(edge[0]) != uf.find_root(edge[1]) {
                uf.union(edge[0], edge[1]);
            } else {
                return edge;
            }
        }

        vec![]
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        [2, 3]
    );
}
