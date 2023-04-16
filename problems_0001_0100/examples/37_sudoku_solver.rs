/*
https://leetcode.com/problems/sudoku-solver/

Write a program to solve a Sudoku puzzle by filling the empty cells.

A sudoku solution must satisfy all of the following rules:

Each of the digits 1-9 must occur exactly once in each row.
Each of the digits 1-9 must occur exactly once in each column.
Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
The '.' character indicates empty cells.

Example 1:

Input: board = [
["5","3",".",".","7",".",".",".","."],
["6",".",".","1","9","5",".",".","."],
[".","9","8",".",".",".",".","6","."],
["8",".",".",".","6",".",".",".","3"],
["4",".",".","8",".","3",".",".","1"],
["7",".",".",".","2",".",".",".","6"],
[".","6",".",".",".",".","2","8","."],
[".",".",".","4","1","9",".",".","5"],
[".",".",".",".","8",".",".","7","9"]]
Output: [
["5","3","4","6","7","8","9","1","2"],
["6","7","2","1","9","5","3","4","8"],
["1","9","8","3","4","2","5","6","7"],
["8","5","9","7","6","1","4","2","3"],
["4","2","6","8","5","3","7","9","1"],
["7","1","3","9","2","4","8","5","6"],
["9","6","1","5","3","7","2","8","4"],
["2","8","7","4","1","9","6","3","5"],
["3","4","5","2","8","6","1","7","9"]]

Explanation: The input board is shown above and the only valid solution.

Constraints:

board.length == 9
board[i].length == 9
board[i][j] is a digit or '.'.
It is guaranteed that the input board has only one solution.
*/

fn main() {
    let mut board = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ];
    Solution::solve_sudoku(&mut board);
    let solution = vec![
        "534678912",
        "672195348",
        "198342567",
        "859761423",
        "426853791",
        "713924856",
        "961537284",
        "287419635",
        "345286179",
    ];
    for (i, row) in board.iter().enumerate() {
        println!("{} {}", solution[i], String::from_iter(row));
    }
}

struct Solution;

// Solution goes below.

use std::collections::HashSet;

impl Solution {
    fn get_valid_numbers(board: &Vec<Vec<char>>, x: usize, y: usize) -> HashSet<char> {
        let mut numbers: HashSet<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'].into_iter().collect();
        let _xs: usize = x - x % 3;
        let _ys: usize = y - y % 3;
        for i in 0..9 {
            numbers.remove(&board[i][y]);
            numbers.remove(&board[x][i]);
            numbers.remove(&board[_xs + i / 3][_ys + i % 3]);
        }

        numbers
    }

    fn solve_bt(board: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
        let mut nx = 9;
        let mut ny = y;
        'outer:
        for _x in x..9 {
            for _y in ny..9 {
                if board[_x][_y] != '.' {
                    continue
                }
                nx = _x;
                ny = _y;
                break 'outer;
            }
            ny = 0
        }
        if nx == 9 {
            return true
        }

        let valid_numbers = Solution::get_valid_numbers(board, nx, ny);
        if valid_numbers.is_empty() {
            return false
        }
        for n in valid_numbers {
            board[nx][ny] = n;
            if Solution::solve_bt(board, nx, ny) {
                return true;
            }
        }
        board[nx][ny] = '.';
        false
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::solve_bt(board, 0, 0);
    }
}
