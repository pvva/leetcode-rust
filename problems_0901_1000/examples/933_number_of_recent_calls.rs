/*
https://leetcode.com/problems/number-of-recent-calls/

You have a RecentCounter class which counts the number of recent requests within a certain time frame.

Implement the RecentCounter class:

RecentCounter() Initializes the counter with zero recent requests.
int ping(int t) Adds a new request at time t, where t represents some time in milliseconds, and
returns the number of requests that has happened in the past 3000 milliseconds (including the new request). Specifically, return the number of requests that have happened in the inclusive range [t - 3000, t].
It is guaranteed that every call to ping uses a strictly larger value of t than the previous call.

 Example 1:

Input
["RecentCounter", "ping", "ping", "ping", "ping"]
[[], [1], [100], [3001], [3002]]
Output
[null, 1, 2, 3, 3]

Explanation
RecentCounter recentCounter = new RecentCounter();
recentCounter.ping(1);     // requests = [1], range is [-2999,1], return 1
recentCounter.ping(100);   // requests = [1, 100], range is [-2900,100], return 2
recentCounter.ping(3001);  // requests = [1, 100, 3001], range is [1,3001], return 3
recentCounter.ping(3002);  // requests = [1, 100, 3001, 3002], range is [2,3002], return 3

Constraints:

1 <= t <= 10^9
Each test case will call ping with strictly increasing values of t.
At most 10^4 calls will be made to ping.
*/

struct RecentCounterTestCase {
    input: i32,
    output: i32,
}

fn new_recent_counter_test_case(input: i32, output: i32) -> RecentCounterTestCase {
    RecentCounterTestCase { input, output }
}

fn main() {
    let cases = vec![
        new_recent_counter_test_case(1, 1),
        new_recent_counter_test_case(100, 2),
        new_recent_counter_test_case(3001, 3),
        new_recent_counter_test_case(3002, 3),
    ];

    let mut rc = RecentCounter::new();
    for case in cases {
        println!(
            "for input {} expected output is {}, got {}",
            case.input,
            case.output,
            rc.ping(case.input)
        );
    }
}

use std::borrow::BorrowMut;
use std::collections::VecDeque;

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
struct RecentCounter {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        // this version runs in 72 ms
        //        self.queue.push_back(t);
        //        let b = t - 3000;
        //        while self.queue[0] < b {
        //            self.queue.pop_front();
        //        }
        //        self.queue.len() as i32

        // this version runs in 24-40 ms
        let q = self.queue.borrow_mut();
        q.push_back(t);
        while let Some(value) = q.front() {
            if *value < t - 3000 {
                q.pop_front();
            } else {
                break;
            }
        }
        q.len() as i32
    }
}
