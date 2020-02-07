/*
https://leetcode.com/problems/max-points-on-a-line

Given n points on a 2D plane, find the maximum number of points that lie on the same straight line.
Input: [[1,1],[2,2],[3,3]]
Output: 3

Input: [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
Output: 4
*/

struct MaxPointsOnALineTestCase {
    points: Vec<Vec<i32>>,
    result: i32,
}

fn new_mpl_test_case(p: Vec<Vec<i32>>, r: i32) -> MaxPointsOnALineTestCase {
    MaxPointsOnALineTestCase {
        points: p,
        result: r,
    }
}

fn main() {
    let cases = vec![
        new_mpl_test_case(vec![vec![1, 1], vec![2, 2], vec![3, 3]], 3),
        new_mpl_test_case(
            vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4],
            ],
            4,
        ),
        new_mpl_test_case(vec![vec![1, 1], vec![1, 1], vec![1, 1]], 3),
        new_mpl_test_case(vec![vec![1, 1], vec![1, 1], vec![2, 2], vec![2, 2]], 4),
        new_mpl_test_case(
            vec![
                vec![1, 1],
                vec![0, 1],
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![4, 4],
                vec![4, 5],
                vec![5, 5],
            ],
            6,
        ),
    ];

    for case in cases {
        println!(
            "Points: {:?}, expected: {}, result: {}",
            case.points.clone(),
            case.result,
            Solution::max_points(case.points)
        );
    }
}

struct Solution;

// solution goes below

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let l = points.len();
        if l < 3 {
            return l as i32;
        }

        let mut max_p = 2;
        for i in 0..l - 1 {
            let mut same_point = 1;

            for j in i + 1..l {
                if points[i][0] == points[j][0] && points[i][1] == points[j][1] {
                    same_point += 1;
                    continue;
                }

                let mut c_max_p = 2;
                // points i and j are fixed, need to check other points if they form a line with i and j
                for p in 0..l {
                    if p == i || p == j {
                        continue;
                    }

                    if Self::is_line(&points[i], &points[j], &points[p]) {
                        c_max_p += 1;
                    }
                }

                if c_max_p > max_p {
                    max_p = c_max_p;
                }
            }

            if same_point > max_p {
                max_p = same_point;
            }
        }

        max_p
    }

    fn is_line(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> bool {
        if p1[0] == p2[0] && p1[1] == p2[1] {
            return true;
        }

        return (p3[0] - p1[0]) as i64 * (p2[1] - p1[1]) as i64
            == (p3[1] - p1[1]) as i64 * (p2[0] - p1[0]) as i64;
    }
}
