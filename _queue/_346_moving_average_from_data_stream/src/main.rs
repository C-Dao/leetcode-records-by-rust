/*
 * @lc app=leetcode id=346 lang=rust
 *
 * [346] Moving average from data stream
 *
 *  https://leetcode.com/problems/moving-average-from-data-stream/
 *
 * algorithms
 * Easy
 * Likes:
 * Dislikes:
 * Total Accepted:
 * Total Submissions:
 * Testcase Example:  ' ["MovingAverage", "next", "next", "next", "next"]\n[[3], [1], [10], [3], [5]]'
 *
 * Given a stream of integers and a window size, calculate the moving average
 * of all integers in the sliding window.
 *
 * Implement the MovingAverage class:
 *
 * MovingAverage(int size) Initializes the object with the size of the window size.
 * double next(int val) Returns the moving average of the last size values of the stream.
 *
 * Example 1:
 *
 *
 * Input
 * ["MovingAverage", "next", "next", "next", "next"]
 * [[3], [1], [10], [3], [5]]
 * Output
 * [null, 1.0, 5.5, 4.66667, 6.0]
 *
 * Explanation
 * MovingAverage movingAverage = new MovingAverage(3);
 * movingAverage.next(1); // return 1.0 = 1 / 1
 * movingAverage.next(10); // return 5.5 = (1 + 10) / 2
 * movingAverage.next(3); // return 4.66667 = (1 + 10 + 3) / 3
 * movingAverage.next(5); // return 6.0 = (10 + 3 + 5) / 3
 *
 *
 *
 * Constraints:
 *
 * 1 <= size <= 1000
 *  -10^5 <= val <= 10^5
 *  At most 10^4 calls will be made to next.
 *
 */

// @lc code=start
use std::collections::VecDeque;
struct MovingAverage {
    queue: VecDeque<i32>,
    capacity: i32,
    sum: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            queue: VecDeque::new(),
            capacity: size,
            sum: 0,
        }
    }

    fn next(&mut self, val: i32) -> f32 {
        self.sum += val;
        if self.queue.len() == self.capacity as usize {
            self.sum -= self.queue.pop_front().unwrap();
        }
        self.queue.push_back(val);
        self.sum as f32 / self.queue.len() as f32
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let param_1 = obj.next(val);
 */
// @lc code=end

fn main() {
    let mut moving_average = MovingAverage::new(3);
    assert_eq!(moving_average.next(1), 1f32);
    assert_eq!(moving_average.next(10), 5.5f32);
    assert_eq!(moving_average.next(3), 4.6666665f32);
    assert_eq!(moving_average.next(5), 6f32);
}
