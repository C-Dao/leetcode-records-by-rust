/*
* @lc app=leetcode id=1061 lang=rust
*
* [1061] Lexicographically Smallest Equivalent String
*
* https://leetcode.com/problems/lexicographically-smallest-equivalent-string/
*
* algorithms
* Medium (77.2%)
* Likes:    1945
* Dislikes: 133
* Total Accepted:    54.8K
* Total Submissions: 71K
* Testcase Example:  ''parker'\n'morris'\n'parser''
*
* "
* You are given two strings of the same length s1 and s2 and a string baseStr.
* We say s1[i] and s2[i] are equivalent characters.
* For example, if s1 = abc and s2 = 'cde', then we have 'a' == 'c', 'b' == 'd', and 'c' == 'e'.
* Equivalent characters follow the usual rules of any equivalence relation:
*
* Reflexivity: 'a' == 'a'.
* Symmetry: 'a' == 'b' implies 'b' == 'a'.
* Transitivity: 'a' == 'b' and 'b' == 'c' implies 'a' == 'c'.
* For example, given the equivalency information from s1 = 'abc' and s2 = 'cde', 'acd' and 'aab' are equivalent
* strings of baseStr = 'eed', and 'aab' is the lexicographically smallest equivalent string of baseStr.
*
* Return the lexicographically smallest equivalent string of baseStr by using the equivalency information from s1 and s2.
*

* Example 1:
*
* Input: s1 = 'parker', s2 = 'morris', baseStr = 'parser'
* Output: 'makkek'
* Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [m,p], [a,o], [k,r,s], [e,i].
* The characters in each group are equivalent and sorted in lexicographical order.So the answer is 'makkek'.
*
*
*
* Example 2:
*
* Input: s1 = 'hello', s2 = 'world', baseStr = 'hold'
* Output: 'hdld'
* Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [h,w], [d,e,o], [l,r].
* So only the second letter 'o' in baseStr is changed to 'd', the answer is 'hdld'.
*
*
*
* Example 3:
*
* Input: s1 = 'leetcode', s2 = 'programs', baseStr = 'sourcecode'
* Output: 'aauaaaaada'
* Explanation: We group the equivalent characters in s1 and s2 as [a,o,e,r,s,c], [l,p], [g,t] and [d,m], thus all letters
* in baseStr except 'u' and 'd' are transformed to 'a', the answer is 'aauaaaaada'.
*
*
* Constraints:
*
*
* 1 <= s1.length,
* s2.length, baseStr <= 1000
* s1.length == s2.length
* s1, s2, and baseStr consist of lowercase English letters.
*
*
*/

struct Solution {}

// @lc code=start
use std::collections::VecDeque;

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
            if root_of_i > root_of_j {
                self.root[root_of_i as usize] = root_of_j;
            } else {
                self.root[root_of_j as usize] = root_of_i;
            }
            return true;
        }

        return false;
    }
}

impl Solution {
    /** union find */
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let base_str_bytes = base_str.as_bytes();
        let mut union_find = UnionFind::new(26);

        for i in 0..s1_bytes.len() {
            union_find.union((s1_bytes[i] - b'a') as i32, (s2_bytes[i] - b'a') as i32);
        }

        let mut ans = vec![];

        for i in 0..base_str.len() {
            ans.push(union_find.find_root((base_str_bytes[i] - b'a') as i32) as u8 + b'a');
        }

        String::from_utf8(ans).unwrap()
    }

    /** dfs */
    pub fn smallest_equivalent_string_dfs(s1: String, s2: String, base_str: String) -> String {
        let mut adj = vec![vec![-0; 26]; 26];
        let mut mapping_char = vec![-1; 26];
        let mut visited = vec![false; 26];
        let mut ans = vec![];

        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let base_str_bytes = base_str.as_bytes();

        for i in 0..s1.len() {
            adj[(s1_bytes[i] - b'a') as usize][(s2_bytes[i] - b'a') as usize] = 1;
            adj[(s2_bytes[i] - b'a') as usize][(s1_bytes[i] - b'a') as usize] = 1;
        }

        for i in 0..26 {
            mapping_char[i] = i as i32;
        }

        for i in 0..26 {
            if !visited[i] {
                let mut component = vec![];
                let mut min_char = 27;

                Self::dfs(i as i32, &adj, &mut visited, &mut component, &mut min_char);

                for j in 0..component.len() {
                    mapping_char[component[j] as usize] = min_char;
                }
            }
        }

        for i in 0..base_str.len() {
            ans.push(mapping_char[(base_str_bytes[i] - b'a') as usize] as u8 + b'a');
        }

        String::from_utf8(ans).unwrap()
    }

    fn dfs(
        src: i32,
        adj: &Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
        component: &mut Vec<i32>,
        min_char: &mut i32,
    ) {
        visited[src as usize] = true;
        component.push(src);
        *min_char = i32::min(*min_char, src);

        for i in 0..26 {
            if adj[src as usize][i] == 1 && !visited[i] {
                Self::dfs(i as i32, adj, visited, component, min_char);
            }
        }
    }

    pub fn smallest_equivalent_string_bfs(s1: String, s2: String, base_str: String) -> String {
        let mut adj = vec![vec![0; 26]; 26];
        let mut mapping_char = vec![0; 26];
        let mut visited = vec![false; 26];
        let mut ans = vec![];

        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let base_str_bytes = base_str.as_bytes();

        for i in 0..s1.len() {
            adj[(s1_bytes[i] - b'a') as usize][(s2_bytes[i] - b'a') as usize] = 1;
            adj[(s2_bytes[i] - b'a') as usize][(s1_bytes[i] - b'a') as usize] = 1;
        }

        for i in 0..26 {
            mapping_char[i] = i as i32;
        }

        for i in 0..26 {
            if !visited[i] {
                let mut component = vec![];
                let mut min_char = 27;

                Self::bfs(i as i32, &adj, &mut visited, &mut component, &mut min_char);

                for j in 0..component.len() {
                    mapping_char[component[j] as usize] = min_char;
                }
            }
        }

        for i in 0..base_str.len() {
            ans.push(mapping_char[(base_str_bytes[i] - b'a') as usize] as u8 + b'a');
        }

        String::from_utf8(ans).unwrap()
    }
    fn bfs(
        src: i32,
        adj: &Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
        component: &mut Vec<i32>,
        min_char: &mut i32,
    ) {
        let mut queue = VecDeque::new();

        queue.push_back(src);

        while !queue.is_empty() {
            let t = queue.pop_front().unwrap();

            visited[t as usize] = true;
            component.push(t);
            *min_char = i32::min(*min_char, t);

            for f in 0..26 {
                if adj[t as usize][f] == 1 && !visited[f] {
                    queue.push_back(f as i32);
                }
            }
        }
    }
}
// @lc code=end
fn main() {
    assert_eq!(
        Solution::smallest_equivalent_string(
            "parker".to_string(),
            "morris".to_string(),
            "parser".to_string()
        ),
        "makkek"
    );

    assert_eq!(
        Solution::smallest_equivalent_string_dfs(
            "parker".to_string(),
            "morris".to_string(),
            "parser".to_string()
        ),
        "makkek"
    );

    assert_eq!(
        Solution::smallest_equivalent_string_bfs(
            "parker".to_string(),
            "morris".to_string(),
            "parser".to_string()
        ),
        "makkek"
    );
}
