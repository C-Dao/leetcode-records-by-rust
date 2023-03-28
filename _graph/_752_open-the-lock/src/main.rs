/*
 * @lc app=leetcode id=752 lang=rust
 *
 * [752] Open the Lock
 *
 * https://leetcode.com/problems/open-the-lock/description/
 *
 * algorithms
 * Medium (55.40%)
 * Likes:    3493
 * Dislikes: 127
 * Total Accepted:    195.9K
 * Total Submissions: 352.3K
 * Testcase Example:  '["0201","0101","0102","1212","2002"]\n"0202"'
 *
 * You have a lock in front of you with 4 circular wheels. Each wheel has 10
 * slots: '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'. The wheels can
 * rotate freely and wrap around: for example we can turn '9' to be '0', or '0'
 * to be '9'. Each move consists of turning one wheel one slot.
 *
 * The lock initially starts at '0000', a string representing the state of the
 * 4 wheels.
 *
 * You are given a list of deadends dead ends, meaning if the lock displays any
 * of these codes, the wheels of the lock will stop turning and you will be
 * unable to open it.
 *
 * Given a target representing the value of the wheels that will unlock the
 * lock, return the minimum total number of turns required to open the lock, or
 * -1 if it is impossible.
 *
 *
 * Example 1:
 *
 *
 * Input: deadends = ["0201","0101","0102","1212","2002"], target = "0202"
 * Output: 6
 * Explanation:
 * A sequence of valid moves would be "0000" -> "1000" -> "1100" -> "1200" ->
 * "1201" -> "1202" -> "0202".
 * Note that a sequence like "0000" -> "0001" -> "0002" -> "0102" -> "0202"
 * would be invalid,
 * because the wheels of the lock become stuck after the display becomes the
 * dead end "0102".
 *
 *
 * Example 2:
 *
 *
 * Input: deadends = ["8888"], target = "0009"
 * Output: 1
 * Explanation: We can turn the last wheel in reverse to move from "0000" ->
 * "0009".
 *
 *
 * Example 3:
 *
 *
 * Input: deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"],
 * target = "8888"
 * Output: -1
 * Explanation: We cannot reach the target without getting stuck.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= deadends.length <= 500
 * deadends[i].length == 4
 * target.length == 4
 * target will not be in the list deadends.
 * target and deadends[i] consist of digits only.
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut dead_set = HashSet::new();
        let mut len = 0;

        for deadend in deadends {
            dead_set.insert(deadend);
        }

        if dead_set.contains(&"0000".to_string()) || dead_set.contains(&target) {
            return -1;
        }

        queue.push_back(format!("0000"));

        while queue.len() > 0 {
            let queue_size = queue.len();

            for _ in 0..queue_size {
                let code = queue.pop_front().unwrap();
                let next_codes = Self::get_next_codes(&code);

                if code == target {
                    return len;
                }

                for next_code in next_codes {
                    if !dead_set.contains(&next_code) && !visited.contains(&next_code) {
                        queue.push_back(next_code.clone());
                        visited.insert(next_code);
                    }
                }
            }
            len += 1;
        }

        -1
    }

    fn get_next_codes(code: &str) -> Vec<String> {
        let mut next_codes = vec![];
        let mut codes = code.as_bytes().to_vec();
        for i in 0..4 {
            let row = codes[i];
            codes[i] = if row == b'0' { b'9' } else { row - 1 };
            next_codes.push(String::from_utf8(codes.clone()).unwrap());
            codes[i] = if row == b'9' { b'0' } else { row + 1 };
            next_codes.push(String::from_utf8(codes.clone()).unwrap());
            codes[i] = row;
        }

        next_codes
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::open_lock(
            vec![
                format!("0201"),
                format!("0101"),
                format!("0102"),
                format!("1212"),
                format!("2002")
            ],
            format!("0202")
        ),
        6
    );
}
