/*
 * @lc app=leetcode id=933 lang=rust
 *
 * [933] Number of Recent Calls
 *
 * https://leetcode.com/problems/number-of-recent-calls/description/
 *
 * algorithms
 * Easy (73.13%)
 * Likes:    870
 * Dislikes: 2589
 * Total Accepted:    134.6K
 * Total Submissions: 183.9K
 * Testcase Example:  '["RecentCounter","ping","ping","ping","ping"]\n[[],[1],[100],[3001],[3002]]'
 *
 * You have a RecentCounter class which counts the number of recent requests
 * within a certain time frame.
 *
 * Implement the RecentCounter class:
 *
 *
 * RecentCounter() Initializes the counter with zero recent requests.
 * int ping(int t) Adds a new request at time t, where t represents some time
 * in milliseconds, and returns the number of requests that has happened in the
 * past 3000 milliseconds (including the new request). Specifically, return the
 * number of requests that have happened in the inclusive range [t - 3000,
 * t].
 *
 *
 * It is guaranteed that every call to ping uses a strictly larger value of t
 * than the previous call.
 *
 *
 * Example 1:
 *
 *
 * Input
 * ["RecentCounter", "ping", "ping", "ping", "ping"]
 * [[], [1], [100], [3001], [3002]]
 * Output
 * [null, 1, 2, 3, 3]
 *
 * Explanation
 * RecentCounter recentCounter = new RecentCounter();
 * recentCounter.ping(1);     // requests = [1], range is [-2999,1], return 1
 * recentCounter.ping(100);   // requests = [1, 100], range is [-2900,100],
 * return 2
 * recentCounter.ping(3001);  // requests = [1, 100, 3001], range is [1,3001],
 * return 3
 * recentCounter.ping(3002);  // requests = [1, 100, 3001, 3002], range is
 * [2,3002], return 3
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= t <= 10^9
 * Each test case will call ping with strictly increasing values of t.
 * At most 10^4 calls will be made to ping.
 *
 *
 */

// @lc code=start
use std::collections::VecDeque;
struct RecentCounter {
    queue: VecDeque<i32>,
    sliding_window_size: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            sliding_window_size: 3000,
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while self.queue.front().is_some() && *self.queue.front().unwrap() < t - self.sliding_window_size {
            self.queue.pop_front();
        }
        self.queue.push_back(t);
        self.queue.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
// @lc code=end

fn main() {
    let mut recent_counter = RecentCounter::new();
    assert_eq!(recent_counter.ping(1), 1);
    assert_eq!(recent_counter.ping(100), 2);
    assert_eq!(recent_counter.ping(3001), 3);
    assert_eq!(recent_counter.ping(3002), 3);
}
