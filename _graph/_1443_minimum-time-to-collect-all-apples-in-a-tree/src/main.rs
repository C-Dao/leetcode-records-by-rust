/*
* @lc app=leetcode id=1443 lang=rust
*
* [1443] Minimum Time to Collect All Apples in a Tree
*
* https://leetcode.com/problems/minimum-time-to-collect-all-apples-in-a-tree/description/
*
* algorithms
* Medium (55.79%)
* Likes:    2843
* Dislikes: 228
* Total Accepted:    79.9K
* Total Submissions: 127.5K
* Testcase Example:  '7\n' +
 '[[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]]\n' +
 '[false,false,true,false,true,true,false]'
*
* Given an undirected tree consisting of n vertices numbered from 0 to n-1,
* which has some apples in their vertices. You spend 1 second to walk over one
* edge of the tree. Return the minimum time in seconds you have to spend to
* collect all apples in the tree, starting at vertex 0 and coming back to this
* vertex.
*
* The edges of the undirected tree are given in the array edges, where
* edges[i] = [ai, bi] means that exists an edge connecting the vertices ai and
* bi. Additionally, there is a boolean array hasApple, where hasApple[i] =
* true means that vertex i has an apple; otherwise, it does not have any
* apple.
*
*
* Example 1:
*
*
* Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple =
* [false,false,true,false,true,true,false]
* Output: 8
* Explanation: The figure above represents the given tree where red vertices
* have an apple. One optimal path to collect all apples is shown by the green
* arrows.
*
*
* Example 2:
*
*
* Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple =
* [false,false,true,false,false,true,false]
* Output: 6
* Explanation: The figure above represents the given tree where red vertices
* have an apple. One optimal path to collect all apples is shown by the green
* arrows.
*
*
* Example 3:
*
*
* Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple =
* [false,false,false,false,false,false,false]
* Output: 0
*
*
*
* Constraints:
*
*
* 1 <= n <= 10^5
* edges.length == n - 1
* edges[i].length == 2
* 0 <= ai < bi <= n - 1
* fromi < toi
* hasApple.length == n
*
*
*/

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut adj: Vec<Vec<i32>> = vec![vec![]; n as usize];

        for edge in edges {
            adj[edge[0] as usize].push(edge[1]);
            adj[edge[1] as usize].push(edge[0]);
        }
        Self::dfs(0, -1, &adj, &has_apple)
    }

    fn dfs(node: i32, parent: i32, adj: &Vec<Vec<i32>>, has_apple: &Vec<bool>) -> i32 {
        let mut total_time = 0;

        for child in adj[node as usize].iter() {
            if *child == parent {
                continue;
            };

            let child_time = Self::dfs(*child, node, adj, has_apple);

            if child_time != 0 || has_apple[*child as usize] {
                total_time += child_time + 2;
            };
        }

        total_time
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, true, false, true, true, false]
        ),
        8
    );
}
