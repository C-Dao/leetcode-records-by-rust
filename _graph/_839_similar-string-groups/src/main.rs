/*
 * @lc app=leetcode id=839 lang=rust
 *
 * [839] Similar String Groups
 *
 * https://leetcode.com/problems/similar-string-groups/description/
 *
 * algorithms
 * Hard (46.54%)
 * Likes:    1037
 * Dislikes: 182
 * Total Accepted:    63.5K
 * Total Submissions: 132.5K
 * Testcase Example:  '["tars","rats","arts","star"]'
 *
 * Two strings X and Y are similar if we can swap two letters (in different
 * positions) of X, so that it equals Y. Also two strings X and Y are similar
 * if they are equal.
 *
 * For example, "tars" and "rats" are similar (swapping at positions 0 and 2),
 * and "rats" and "arts" are similar, but "star" is not similar to "tars",
 * "rats", or "arts".
 *
 * Together, these form two connected groups by similarity: {"tars", "rats",
 * "arts"} and {"star"}.  Notice that "tars" and "arts" are in the same group
 * even though they are not similar.  Formally, each group is such that a word
 * is in the group if and only if it is similar to at least one other word in
 * the group.
 *
 * We are given a list strs of strings where every string in strs is an anagram
 * of every other string in strs. How many groups are there?
 *
 *
 * Example 1:
 *
 *
 * Input: strs = ["tars","rats","arts","star"]
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: strs = ["omv","ovm"]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= strs.length <= 300
 * 1 <= strs[i].length <= 300
 * strs[i] consists of lowercase letters only.
 * All words in strs have the same length and are anagrams of each other.
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
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut uf = UnionFind::new(strs.len() as i32);

        for i in 0..strs.len() {
            for j in (i + 1)..strs.len() {
                if Self::check(&strs[i], &strs[j]) {
                    uf.union(i as i32, j as i32);
                }
            }
        }
        uf.root
            .iter()
            .enumerate()
            .filter(|(i, _)| uf.root[*i] == *i as i32)
            .collect::<Vec<(_, _)>>()
            .len() as i32
    }

    fn check(a: &str, b: &str) -> bool {
        let mut diff = 0;
        let (a, b) = (a.as_bytes(), b.as_bytes());

        for i in 0..a.len() {
            if a[i] != b[i] {
                diff += 1;

                if diff > 2 {
                    return false;
                }
            }
        }

        true
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::num_similar_groups(vec![
            "tars".to_string(),
            "rats".to_string(),
            "arts".to_string(),
            "star".to_string()
        ]),
        2
    );
}
