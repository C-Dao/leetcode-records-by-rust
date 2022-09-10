/*
 * @lc app=leetcode id=941 lang=rust
 *
 * [941] Valid Mountain Array
 *
 * https://leetcode.com/problems/valid-mountain-array/description/
 *
 * algorithms
 * Easy (33.64%)
 * Likes:    2219
 * Dislikes: 146
 * Total Accepted:    308.5K
 * Total Submissions: 917.3K
 * Testcase Example:  '[2,1]'
 *
 * Given an array of integers arr, return true if and only if it is a valid
 * mountain array.
 * 
 * Recall that arr is a mountain array if and only if:
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
 * 
 * Example 1:
 * Input: arr = [2,1]
 * Output: false
 * Example 2:
 * Input: arr = [3,5,5]
 * Output: false
 * Example 3:
 * Input: arr = [0,3,2,1]
 * Output: true
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= arr.length <= 10^4
 * 0 <= arr[i] <= 10^4
 * 
 * 
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut up_down = 0;
        if arr.len() < 3 {
            false
        } else {
            for i in 0..arr.len() - 1 {
                if i == 0 && arr[i] < arr[i + 1] {
                    up_down = 1;
                    continue;
                }
                if arr[i] < arr[i + 1] && up_down == 1 {
                    continue;
                }

                if arr[i] > arr[i + 1] && up_down == 1 {
                    up_down = 2;
                    continue;
                }

                if arr[i] > arr[i + 1] && up_down == 2 {
                    continue;
                }

                up_down = 3;
                break;
            }
            if up_down == 2 {
                true
            } else {
                false
            }
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
    assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
    assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
    assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 3]), false);
}
