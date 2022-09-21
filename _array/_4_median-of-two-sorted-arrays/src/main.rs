/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 *
 * https://leetcode.com/problems/median-of-two-sorted-arrays/description/
 *
 * algorithms
 * Hard (34.51%)
 * Likes:    19576
 * Dislikes: 2236
 * Total Accepted:    1.6M
 * Total Submissions: 4.5M
 * Testcase Example:  '[1,3]\n[2]'
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return
 * the median of the two sorted arrays.
 *
 * The overall run time complexity should be O(log (m+n)).
 *
 *
 * Example 1:
 *
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 *
 * Example 2:
 *
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 *
 *
 * Constraints:
 *
 *
 * nums1.length == m
 * nums2.length == n
 * 0 <= m <= 1000
 * 0 <= n <= 1000
 * 1 <= m + n <= 2000
 * -10^6 <= nums1[i], nums2[i] <= 10^6
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        if len % 2 == 1 {
            Self::get_k_element((len + 1) / 2, &nums1, &nums2)
        } else {
            (Self::get_k_element(len / 2, &nums1, &nums2)
                + Self::get_k_element(len / 2 + 1, &nums1, &nums2))
                / 2.0
        }
    }

    fn get_k_element(mut k: usize, nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
        let (len_1, len_2) = (nums1.len(), nums2.len());
        let (mut index_1, mut index_2): (usize, usize) = (0, 0);

        loop {
            if index_1 == len_1 {
                return nums2[index_2 + k - 1] as f64;
            };

            if index_2 == len_2 {
                return nums1[index_1 + k - 1] as f64;
            };

            if k == 1 {
                return f64::min(nums1[index_1] as f64, nums2[index_2] as f64);
            };

            let next_index_1 = usize::min(index_1 + k / 2 - 1, len_1 - 1);
            let next_index_2 = usize::min(index_2 + k / 2 - 1, len_2 - 1);

            let pivot_1 = nums1[next_index_1];
            let pivot_2 = nums2[next_index_2];

            if pivot_1 <= pivot_2 {
                k -= next_index_1 - index_1 + 1;
                index_1 = next_index_1 + 1;
            } else {
                k -= next_index_2 - index_2 + 1;
                index_2 = next_index_2 + 1;
            };
        }
    }
}
// @lc code=end

fn main() {
 assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
}
