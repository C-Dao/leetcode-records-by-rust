/*
 * @lc app=leetcode id=47 lang=rust
 *
 * [47] Permutations II
 *
 * https://leetcode.com/problems/permutations-ii/description/
 *
 * algorithms
 * Medium (55.86%)
 * Likes:    6053
 * Dislikes: 107
 * Total Accepted:    676.9K
 * Total Submissions: 1.2M
 * Testcase Example:  '[1,1,2]'
 *
 * Given a collection of numbers, nums, that might contain duplicates, return
 * all possible unique permutations in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,1,2]
 * Output:
 * [[1,1,2],
 * ⁠[1,2,1],
 * ⁠[2,1,1]]
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 8
 * -10 <= nums[i] <= 10
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(0, &mut nums, &mut ans);

        ans
    }

    fn dfs(index: usize, nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            ans.push(nums.clone());
        } else {
            let mut used = Vec::with_capacity(nums.len());
            for i in index..nums.len() {
                if !used.iter().any(|x| *x == nums[i]) {
                    used.push(nums[i]);
                    nums.swap(index, i);
                    Self::dfs(index + 1, nums, ans);
                    nums.swap(index, i);
                }
            }
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::permute_unique(vec![1, 1, 2]),
        [[1, 1, 2], [1, 2, 1], [2, 1, 1]]
    );
}
