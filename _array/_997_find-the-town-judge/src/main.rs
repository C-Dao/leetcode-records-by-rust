/*
 * @lc app=leetcode id=997 lang=rust
 *
 * [997] Find the Town Judge
 *
 * https://leetcode.com/problems/find-the-town-judge/description/
 *
 * algorithms
 * Easy (49.64%)
 * Likes:    5054
 * Dislikes: 403
 * Total Accepted:    374.9K
 * Total Submissions: 755.4K
 * Testcase Example:  '2\n[[1,2]]'
 *
 * In a town, there are n people labeled from 1 to n. There is a rumor that one
 * of these people is secretly the town judge.
 *
 * If the town judge exists, then:
 *
 *
 * The town judge trusts nobody.
 * Everybody (except for the town judge) trusts the town judge.
 * There is exactly one person that satisfies properties 1 and 2.
 *
 *
 * You are given an array trust where trust[i] = [ai, bi] representing that the
 * person labeled ai trusts the person labeled bi.
 *
 * Return the label of the town judge if the town judge exists and can be
 * identified, or return -1 otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 2, trust = [[1,2]]
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: n = 3, trust = [[1,3],[2,3]]
 * Output: 3
 *
 *
 * Example 3:
 *
 *
 * Input: n = 3, trust = [[1,3],[2,3],[3,1]]
 * Output: -1
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 1000
 * 0 <= trust.length <= 10^4
 * trust[i].length == 2
 * All the pairs of trust are unique.
 * ai != bi
 * 1 <= ai, bi <= n
 *
 *
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut count_map = vec![1; n as usize];

        for i in 0..trust.len() {
            count_map[trust[i][1] as usize - 1] += 1;
            count_map[trust[i][0] as usize - 1] = 0;
        }

        let ans = count_map.iter().enumerate().find(|(_, &x)| x == n);

        if ans.is_some() {
            (ans.unwrap().0 + 1) as i32
        } else {
            -1
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::find_judge(2, vec![vec![1, 2], vec![2, 1]]), -1);
}
