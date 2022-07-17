/*
 * @lc app=leetcode id=941 lang=rust
 *
 * [941] Valid Mountain Array
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
