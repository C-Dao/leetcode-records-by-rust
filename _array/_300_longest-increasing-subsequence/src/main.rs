/*
 * @lc app=leetcode id=300 lang=rust
 *
 * [300] Longest Increasing Subsequence
 *
 * https://leetcode.com/problems/longest-increasing-subsequence/description/
 *
 * algorithms
 * Medium (50.03%)
 * Likes:    15621
 * Dislikes: 282
 * Total Accepted:    1.1M
 * Total Submissions: 2.1M
 * Testcase Example:  '[10,9,2,5,3,7,101,18]'
 *
 * Given an integer array nums, return the length of the longest strictly
 * increasing subsequence.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [10,9,2,5,3,7,101,18]
 * Output: 4
 * Explanation: The longest increasing subsequence is [2,3,7,101], therefore
 * the length is 4.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,1,0,3,2,3]
 * Output: 4
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [7,7,7,7,7,7,7]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 2500
 * -10^4 <= nums[i] <= 10^4
 *
 *
 *
 * Follow up: Can you come up with an algorithm that runs in O(n log(n)) time
 * complexity?
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn length_of_lis_another_edition(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        };
        let mut dp = vec![1; nums.len()];

        let mut max_count = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = i32::max(dp[i], dp[j] + 1);
                }
            }
            max_count = i32::max(max_count, dp[i]);
        }
        return max_count;
    }

    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        };
        let mut ans = vec![nums[0]];

        for i in 0..nums.len() {
            if nums[i] > ans[ans.len() - 1] {
                ans.push(nums[i]);
            } else {
                let (mut start, mut end) = (0, ans.len() - 1);
                while start < end {
                    let mid = (start + end) / 2;
                    if ans[mid] < nums[i] {
                        start = mid + 1;
                    } else {
                        end = mid;
                    }
                }
                ans[start] = nums[i];
            }
        }

        ans.len() as i32
    }

    pub fn get_longest_increasing_subsequence_sub_index(nums: Vec<i32>) -> Vec<usize> {
        let mut link_list = vec![0; nums.len()];
        let mut ans_indexs = vec![0];

        for i in 0..nums.len() {
            let j = ans_indexs[ans_indexs.len() - 1];
            if nums[j] < nums[i] {
                link_list[i] = j;
                ans_indexs.push(i);
            } else {
                let (mut start, mut end) = (0, ans_indexs.len() - 1);
                while start < end {
                    let mid = (start + end) / 2;
                    if nums[ans_indexs[mid]] < nums[i] {
                        start = mid + 1;
                    } else {
                        end = mid;
                    }
                }
                if nums[i] < nums[ans_indexs[start]] {
                    if start > 0 {
                        link_list[i] = ans_indexs[start - 1];
                    }
                    ans_indexs[start] = i;
                }
            }
        }

        let mut val = ans_indexs[ans_indexs.len() - 1];
        for i in (0..ans_indexs.len()).rev() {
            ans_indexs[i] = val;
            val = link_list[val];
        }

        ans_indexs
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(
        Solution::length_of_lis_another_edition(vec![0, 1, 0, 3, 2, 3]),
        4
    );
    assert_eq!(
        Solution::get_longest_increasing_subsequence_sub_index(vec![0, 1, 0, 3, 2, 3]),
        vec![0, 1, 4, 5]
    );
}
