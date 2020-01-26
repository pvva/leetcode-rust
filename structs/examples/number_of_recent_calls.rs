/*
Write a class RecentCounter to count recent requests.

It has only one method: ping(int t), where t represents some time in milliseconds.

Return the number of pings that have been made from 3000 milliseconds ago until now.

Any ping with time in [t - 3000, t] will count, including the current ping.

It is guaranteed that every call to ping uses a strictly larger value of t than before.


Example 1:

Input: inputs = ["RecentCounter","ping","ping","ping","ping"], inputs = [[],[1],[100],[3001],[3002]]
Output: [null,1,2,3,3]
*/

fn main() {
    let mut rc = RecentCounter::new();
    println!("{}", rc.ping(1));
    println!("{}", rc.ping(100));
    println!("{}", rc.ping(3001));
    println!("{}", rc.ping(3002));
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
