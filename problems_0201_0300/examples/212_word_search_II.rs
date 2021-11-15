/*
https://leetcode.com/problems/word-search-ii/

Given an m x n board of characters and a list of strings words, return all words on the board.

Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are
horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.

Example:

Input:
board = [
  ['o','a','a','n'],
  ['e','t','a','e'],
  ['i','h','k','r'],
  ['i','f','l','v']
]
words = ["oath","pea","eat","rain"]

Output: ["eat","oath"]

Constraints:
m == board.length
n == board[i].length
1 <= m, n <= 12
board[i][j] is a lowercase English letter.
1 <= words.length <= 3 * 10^4
1 <= words[i].length <= 10
words[i] consists of lowercase English letters.
All the strings of words are unique.
*/

struct WordSearchIITestCase {
    board: Vec<Vec<char>>,
    words: Vec<String>,
    output: Vec<String>,
}

fn new_word_search_ii_test_case(
    board: Vec<Vec<char>>,
    words: Vec<String>,
    output: Vec<String>,
) -> WordSearchIITestCase {
    WordSearchIITestCase {
        board,
        words,
        output,
    }
}

fn main() {
    let cases = vec![
        new_word_search_ii_test_case(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            to_strings(vec!["oath", "pea", "eat", "rain"]),
            to_strings(vec!["eat", "oath"]),
        ),
        new_word_search_ii_test_case(
            vec![vec!['a', 'b'], vec!['a', 'a']],
            to_strings(vec!["aba", "baa", "bab", "aaab", "aaa", "aaaa", "aaba"]),
            to_strings(vec!["?"]),
        ),
    ];

    for case in cases {
        println!(
            "for board {:?} and words {:?} the result should be {:?}, got {:?}",
            case.board.clone(),
            case.words.clone(),
            case.output,
            Solution::find_words(case.board, case.words)
        )
    }

    //    let mut trie = Trie::new();
    //    trie.add_string("aaa");
    //    trie.add_string("aab");
    //
    //    for word in to_strings(vec!["aba", "baa", "bab", "aaab", "aaa", "aaaa", "aaba"]) {
    //        trie.add_string(word);
    //    }
    //
    //    println!("{:?}", trie);
}

struct Solution;

// solution is below

use std::borrow::BorrowMut;
use std::collections::HashMap;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut b = board.clone();
        let mut max_word_length = b.len();
        if max_word_length == 0 {
            return vec![];
        }
        max_word_length *= b[0].len();

        let mut trie = Trie::new();
        for word in words {
            if word.len() <= max_word_length {
                trie.add_string(word);
            }
        }
        let mut result = vec![];

        for x in 0..b.len() {
            for y in 0..b[x].len() {
                for (c, t) in trie.next.iter_mut() {
                    if *c != b[x][y] {
                        continue;
                    }
                    Solution::find_words_dfs(b.borrow_mut(), x, y, t, &mut result);
                }
            }
        }

        result
    }

    fn find_words_dfs(
        board: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        trie: &mut Box<Trie>,
        result: &mut Vec<String>,
    ) {
        let current_char = board[x][y];
        if current_char == '-' {
            return;
        }

        if let Some(word) = trie.word.as_ref().clone() {
            result.push(word.to_string());
            trie.word = None;
        }
        for dir in [(0, -1), (1, 0), (0, 1), (-1, 0)].iter() {
            let nx = x as i32 + dir.0;
            let ny = y as i32 + dir.1;
            if nx < 0 || ny < 0 {
                continue;
            }
            let unx = nx as usize;
            let uny = ny as usize;

            if unx >= board.len() || uny >= board[unx].len() {
                continue;
            }

            let next_possible_char = board[unx][uny];
            if let Some(next_trie) = trie.next.get_mut(&next_possible_char) {
                board[x][y] = '-';
                Solution::find_words_dfs(board, unx, uny, next_trie, result);
                board[x][y] = current_char;
            }
        }
    }
}

fn to_strings(input: Vec<&str>) -> Vec<String> {
    let mut result = vec![];

    for str in input {
        result.push(str.to_string());
    }

    result
}

#[derive(Debug, Default)]
struct Trie {
    word: Option<String>,
    next: HashMap<char, Box<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn add_string<S: Into<String>>(&mut self, s: S) {
        let string = s.into();
        let mut ct = self;
        for c in string.chars() {
            ct = moving(ct)
                .next
                .entry(c)
                .or_insert(Box::new(Self::new()))
                .borrow_mut();
        }
        ct.word = Some(string);
    }
}

fn moving<T>(t: T) -> T {
    t
}
