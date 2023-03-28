/*
* @lc app=leetcode id=399 lang=rust
*
* [399] Evaluate Division
*
* https://leetcode.com/problems/evaluate-division/description/
*
* algorithms
* Medium (58.97%)
* Likes:    6863
* Dislikes: 602
* Total Accepted:    326.6K
* Total Submissions: 547.7K
* Testcase Example:  '[["a","b"],["b","c"]]\n' +
 '[2.0,3.0]\n' +
 '[["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]'
*
* You are given an array of variable pairs equations and an array of real
* numbers values, where equations[i] = [Ai, Bi] and values[i] represent the
* equation Ai / Bi = values[i]. Each Ai or Bi is a string that represents a
* single variable.
*
* You are also given some queries, where queries[j] = [Cj, Dj] represents the
* j^th query where you must find the answer for Cj / Dj = ?.
*
* Return the answers to all queries. If a single answer cannot be determined,
* return -1.0.
*
* Note: The input is always valid. You may assume that evaluating the queries
* will not result in division by zero and that there is no contradiction.
*
*
* Example 1:
*
*
* Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries =
* [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
* Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
* Explanation:
* Given: a / b = 2.0, b / c = 3.0
* queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
* return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
*
*
* Example 2:
*
*
* Input: equations = [["a","b"],["b","c"],["bc","cd"]], values =
* [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
* Output: [3.75000,0.40000,5.00000,0.20000]
*
*
* Example 3:
*
*
* Input: equations = [["a","b"]], values = [0.5], queries =
* [["a","b"],["b","a"],["a","c"],["x","y"]]
* Output: [0.50000,2.00000,-1.00000,-1.00000]
*
*
*
* Constraints:
*
*
* 1 <= equations.length <= 20
* equations[i].length == 2
* 1 <= Ai.length, Bi.length <= 5
* values.length == equations.length
* 0.0 < values[i] <= 20.0
* 1 <= queries.length <= 20
* queries[i].length == 2
* 1 <= Cj.length, Dj.length <= 5
* Ai, Bi, Cj, Dj consist of lower case English letters and digits.
*
*
*/

struct Solution {}

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let graph = Self::build_graph(&equations, &values);
        let mut ans: Vec<f64> = vec![0 as f64; queries.len()];

        for i in 0..queries.len() {
            let from = queries[i].first().unwrap();
            let to = queries[i].last().unwrap();

            if !graph.contains_key(from) || !graph.contains_key(to) {
                ans[i] = -1 as f64;
            } else {
                let mut visited = HashSet::new();
                ans[i] = Self::dfs(&graph, &from, &to, &mut visited);
            }
        }

        ans
    }

    fn build_graph(
        equations: &Vec<Vec<String>>,
        values: &Vec<f64>,
    ) -> HashMap<String, HashMap<String, f64>> {
        let mut graph = HashMap::new();

        for i in 0..equations.len() {
            let a = equations[i].first().unwrap().clone();
            let b = equations[i].last().unwrap().clone();
            if !graph.contains_key(&a) {
                graph.insert(a.clone(), HashMap::new());
            }
            graph.get_mut(&a).unwrap().insert(b.clone(), values[i]);

            if !graph.contains_key(&b) {
                graph.insert(b.clone(), HashMap::new());
            }
            graph
                .get_mut(&b)
                .unwrap()
                .insert(a.clone(), 1.0 / values[i]);
        }

        graph
    }

    fn dfs(
        graph: &HashMap<String, HashMap<String, f64>>,
        from: &String,
        to: &String,
        visited: &mut HashSet<String>,
    ) -> f64 {
        if from == to {
            return 1.0;
        }

        visited.insert(from.clone());

        for (key, value) in graph.get(from).unwrap().iter() {
            if !visited.contains(key) {
                let next_value = Self::dfs(graph, key, to, visited);
                if next_value > 0 as f64 {
                    return value * next_value;
                }
            }
        }

        visited.remove(from);
        -1.0
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::calc_equation(
            vec![
                vec![format!("a"), format!("b")],
                vec![format!("b"), format!("c")]
            ],
            vec![2.0, 3.0],
            vec![
                vec![format!("a"), format!("c")],
                vec![format!("b"), format!("a")],
                vec![format!("a"), format!("e")],
                vec![format!("a"), format!("a")],
                vec![format!("x"), format!("x")]
            ]
        ),
        [6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
    );
}
