/*
 * @lc app=leetcode id=1346 lang=rust
 *
 * [1346] Check If N and Its Double Exist
 *
 * https://leetcode.com/problems/check-if-n-and-its-double-exist/description/
 *
 * algorithms
 * Easy (35.83%)
 * Likes:    1078
 * Dislikes: 132
 * Total Accepted:    215.9K
 * Total Submissions: 602.3K
 * Testcase Example:  '[10,2,5,3]'
 *
 * Given an array arr of integers, check if there exists two integers N and M
 * such that N is the double of M ( i.e. N = 2 * M).
 * 
 * More formally check if there exists two indices i and j such that :
 * 
 * 
 * i != j
 * 0 <= i, j < arr.length
 * arr[i] == 2 * arr[j]
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: arr = [10,2,5,3]
 * Output: true
 * Explanation: N = 10 is the double of M = 5,that is, 10 = 2 * 5.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: arr = [7,1,14,11]
 * Output: true
 * Explanation: N = 14 is the double of M = 7,that is, 14 = 2 * 7.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: arr = [3,1,7,11]
 * Output: false
 * Explanation: In this case does not exist N and M, such that N = 2 * M.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 2 <= arr.length <= 500
 * -10^3 <= arr[i] <= 10^3
 * 
 * 
 */

use std::collections::HashSet;
struct Solution {}

// @lc code=start
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut mask = HashSet::new();
        let mut result = false;
        for i in 0..arr.len() {
            if mask.contains(&(arr[i] * 2)) || (arr[i] % 2 == 0 && mask.contains(&(arr[i] / 2))) {
                result = true;
                break;
            }
            mask.insert(arr[i]);
        }
        result
    }
}
// @lc code=end

fn main() {
    let arr = vec![10, 2, 5, 3];
    assert_eq!(Solution::check_if_exist(arr), true);
}
