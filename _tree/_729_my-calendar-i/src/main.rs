/*
 * @lc app=leetcode id=729 lang=rust
 *
 * [729] My Calendar I
 *
 * https://leetcode.com/problems/my-calendar-i/description/
 *
 * algorithms
 * Medium (55.42%)
 * Likes:    3738
 * Dislikes: 95
 * Total Accepted:    245.4K
 * Total Submissions: 430K
 * Testcase Example:  '["MyCalendar","book","book","book"]\n[[],[10,20],[15,25],[20,30]]'
 *
 * You are implementing a program to use as your calendar. We can add a new
 * event if adding the event will not cause a double booking.
 *
 * A double booking happens when two events have some non-empty intersection
 * (i.e., some moment is common to both events.).
 *
 * The event can be represented as a pair of integers start and end that
 * represents a booking on the half-open interval [start, end), the range of
 * real numbers x such that start <= x < end.
 *
 * Implement the MyCalendar class:
 *
 *
 * MyCalendar() Initializes the calendar object.
 * boolean book(int start, int end) Returns true if the event can be added to
 * the calendar successfully without causing a double booking. Otherwise,
 * return false and do not add the event to the calendar.
 *
 *
 *
 * Example 1:
 *
 *
 * Input
 * ["MyCalendar", "book", "book", "book"]
 * [[], [10, 20], [15, 25], [20, 30]]
 * Output
 * [null, true, false, true]
 *
 * Explanation
 * MyCalendar myCalendar = new MyCalendar();
 * myCalendar.book(10, 20); // return True
 * myCalendar.book(15, 25); // return False, It can not be booked because time
 * 15 is already booked by another event.
 * myCalendar.book(20, 30); // return True, The event can be booked, as the
 * first event takes every time less than 20, but not including 20.
 *
 *
 * Constraints:
 *
 *
 * 0 <= start < end <= 10^9
 * At most 1000 calls will be made to book.
 *
 *
 */

// @lc code=start
use std::collections::BTreeMap;

struct MyCalendar {
    events: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            events: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        // btree_map.floorEntry, entry as Option<(start, end)>, 获取键小于或等于给定值的最大值，如果没有返回 None
        let event = self.events.range(..=start).next_back();
        if event.is_some() && *event.unwrap().1 > start {
            return false;
        }

        // btree_map.ceilingEntry, entry as Option<(start, end)>, 获取键大于或等于给定值的最小值，如果没有返回 None
        let event = self.events.range(start..).next();
        if event.is_some() && *event.unwrap().0 < end {
            return false;
        }

        self.events.insert(start, end);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
// @lc code=end

fn main() {
    let mut my_calendar = MyCalendar::new();

    assert_eq!(my_calendar.book(10, 20), true);
    assert_eq!(my_calendar.book(15, 25), false);
    assert_eq!(my_calendar.book(20, 30), true);
}
