/*
 * @lc app=leetcode id=852 lang=rust
 *
 * [852] Peak Index in a Mountain Array
 *
 * https://leetcode.com/problems/peak-index-in-a-mountain-array/description/
 *
 * algorithms
 * Medium (69.79%)
 * Likes:    4333
 * Dislikes: 1760
 * Total Accepted:    497.5K
 * Total Submissions: 717.6K
 * Testcase Example:  '[0,1,0]'
 *
 * An array arr a mountain if the following properties hold:
 *
 *
 * arr.length >= 3
 * There exists some i with 0 < i < arr.length - 1 such that:
 *
 * arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
 * arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
 *
 *
 *
 *
 * Given a mountain array arr, return the index i such that arr[0] < arr[1] <
 * ... < arr[i - 1] < arr[i] > arr[i + 1] > ... > arr[arr.length - 1].
 *
 * You must solve it in O(log(arr.length)) time complexity.
 *
 *
 * Example 1:
 *
 *
 * Input: arr = [0,1,0]
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input: arr = [0,2,1,0]
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: arr = [0,10,5,2]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * 3 <= arr.length <= 10^5
 * 0 <= arr[i] <= 10^6
 * arr is guaranteed to be a mountain array.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut left, mut right) = (1, arr.len() - 2);

        while left <= right {
            let mid = (left + right) / 2;
            if arr[mid] > arr[mid + 1] && arr[mid] > arr[mid - 1] {
                return mid as i32;
            };

            if arr[mid] > arr[mid - 1] {
                left = mid + 1;
            } else {
                right = mid - 1;
            };
        }

        -1
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
}
