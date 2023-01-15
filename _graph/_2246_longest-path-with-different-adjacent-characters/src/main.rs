/*
 * @lc app=leetcode id=2246 lang=rust
 *
 * [2246] Longest Path With Different Adjacent Characters
 *
 * https://leetcode.com/problems/longest-path-with-different-adjacent-characters/description/
 *
 * algorithms
 * Hard (43.91%)
 * Likes:    1739
 * Dislikes: 45
 * Total Accepted:    47.1K
 * Total Submissions: 86.8K
 * Testcase Example:  '[-1,0,0,1,1,2]\n"abacbe"'
 *
 * You are given a tree (i.e. a connected, undirected graph that has no cycles)
 * rooted at node 0 consisting of n nodes numbered from 0 to n - 1. The tree is
 * represented by a 0-indexed array parent of size n, where parent[i] is the
 * parent of node i. Since node 0 is the root, parent[0] == -1.
 *
 * You are also given a string s of length n, where s[i] is the character
 * assigned to node i.
 *
 * Return the length of the longest path in the tree such that no pair of
 * adjacent nodes on the path have the same character assigned to them.
 *
 *
 * Example 1:
 *
 *
 * Input: parent = [-1,0,0,1,1,2], s = "abacbe"
 * Output: 3
 * Explanation: The longest path where each two adjacent nodes have different
 * characters in the tree is the path: 0 -> 1 -> 3. The length of this path is
 * 3, so 3 is returned.
 * It can be proven that there is no longer path that satisfies the
 * conditions.
 *
 *
 * Example 2:
 *
 *
 * Input: parent = [-1,0,0,0], s = "aabc"
 * Output: 3
 * Explanation: The longest path where each two adjacent nodes have different
 * characters is the path: 2 -> 0 -> 3. The length of this path is 3, so 3 is
 * returned.
 *
 *
 *
 * Constraints:
 *
 *
 * n == parent.length == s.length
 * 1 <= n <= 10^5
 * 0 <= parent[i] <= n - 1 for all i >= 1
 * parent[0] == -1
 * parent represents a valid tree.
 * s consists of only lowercase English letters.
 *
 *
 */
struct Solution {}

// @lc code=start
impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let n = parent.len();
        let mut children = vec![vec![]; n];

        for i in 1..n {
            children[parent[i] as usize].push(i as i32);
        }

        let mut longest_path = 1;

        Self::dfs(0, &mut children, &s.as_bytes(), &mut longest_path);

        longest_path
    }

    fn dfs(current_node: i32, children: &Vec<Vec<i32>>, s: &[u8], longest_path: &mut i32) -> i32 {
        let mut longest_chain = 0;
        let mut second_longest_chain = 0;
        for child in children[current_node as usize].iter() {
            let longest_chain_starting_from_child = Self::dfs(*child, children, s, longest_path);

            if s[current_node as usize] == s[*child as usize] {
                continue;
            }
            if longest_chain_starting_from_child > longest_chain {
                second_longest_chain = longest_chain;
                longest_chain = longest_chain_starting_from_child;
            } else if longest_chain_starting_from_child > second_longest_chain {
                second_longest_chain = longest_chain_starting_from_child;
            }
        }

        *longest_path = i32::max(*longest_path, longest_chain + second_longest_chain + 1);
        return longest_chain + 1;
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string()),
        3
    );
}
