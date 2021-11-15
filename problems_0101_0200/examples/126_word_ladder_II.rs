/*
https://leetcode.com/problems/word-ladder-ii/

A transformation sequence from word beginWord to word endWord using a dictionary wordList is a
sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:

Every adjacent pair of words differs by a single letter.
Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
sk == endWord
Given two words, beginWord and endWord, and a dictionary wordList, return all the shortest
transformation sequences from beginWord to endWord, or an empty list if no such sequence exists.
Each sequence should be returned as a list of the words [beginWord, s1, s2, ..., sk].

Example 1:

Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
Output: [["hit","hot","dot","dog","cog"],["hit","hot","lot","log","cog"]]
Explanation: There are 2 shortest transformation sequences:
"hit" -> "hot" -> "dot" -> "dog" -> "cog"
"hit" -> "hot" -> "lot" -> "log" -> "cog"

Example 2:

Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
Output: []
Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.

Constraints:
1 <= beginWord.length <= 5
endWord.length == beginWord.length
1 <= wordList.length <= 1000
wordList[i].length == beginWord.length
beginWord, endWord, and wordList[i] consist of lowercase English letters.
beginWord != endWord
All the words in wordList are unique.
*/

fn to_strings(input: Vec<&str>) -> Vec<String> {
    let mut result = vec![];

    for str in input {
        result.push(str.to_string());
    }

    result
}

fn to_strings2(input: Vec<Vec<&str>>) -> Vec<Vec<String>> {
    let mut result = vec![];

    for vec in input {
        let mut t = vec![];
        for str in vec {
            t.push(str.to_string());
        }
        result.push(t);
    }

    result
}

struct WordLadderIITestCase {
    begin_word: String,
    end_word: String,
    word_list: Vec<String>,
    output: Vec<Vec<String>>,
}

fn new_word_ladder_ii_test_case(
    begin_word: &str,
    end_word: &str,
    word_list: Vec<&str>,
    output: Vec<Vec<&str>>,
) -> WordLadderIITestCase {
    WordLadderIITestCase {
        begin_word: begin_word.to_string(),
        end_word: end_word.to_string(),
        word_list: to_strings(word_list),
        output: to_strings2(output),
    }
}

fn main() {
    let cases = vec![
        new_word_ladder_ii_test_case(
            "hit",
            "cog",
            vec!["hot", "dot", "dog", "lot", "log"],
            vec![
                vec!["hit", "hot", "dot", "dog", "cog"],
                vec!["hit", "hot", "lot", "log", "cog"],
            ],
        ),
        new_word_ladder_ii_test_case(
            "hit",
            "cog",
            vec!["hot", "dot", "dog", "lot", "log", "cog"],
            vec![
                vec!["hit", "hot", "dot", "dog", "cog"],
                vec!["hit", "hot", "lot", "log", "cog"],
            ],
        ),
        new_word_ladder_ii_test_case(
            "hit",
            "cog",
            vec!["hot", "dot", "dog", "lot", "log"],
            vec![],
        ),
    ];

    for case in cases {
        println!(
            "from '{}' to '{}' through {:?} is expected to be {:?}, got {:?}",
            case.begin_word.clone(),
            case.end_word.clone(),
            case.word_list.clone(),
            case.output,
            Solution::find_ladders(case.begin_word, case.end_word, case.word_list)
        )
    }
}

struct Solution;

// solution goes below

use std::collections::HashSet;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut words = HashSet::new();
        for word in word_list {
            if word.eq(&begin_word) {
                continue;
            }
            words.insert(word);
        }

        let mut stack: Vec<Vec<String>> = vec![vec![begin_word]];
        let mut found_shortest = false;
        while !stack.is_empty() && !found_shortest {
            let mut total_next: Vec<Vec<String>> = vec![];
            let mut current_level = HashSet::new();

            for path in &stack {
                if let Some(last_word) = path.last() {
                    words.remove(last_word);
                    current_level.insert(last_word);
                }
            }

            for path in &stack {
                if let Some(from) = path.last() {
                    let (next, found) = Self::execute_bfs_with_results(from, &end_word, &words);

                    if found {
                        found_shortest = true;
                    }

                    for step in next {
                        if (found_shortest && !step.eq(&end_word)) || current_level.contains(&step)
                        {
                            continue;
                        }
                        let mut temp_path = path.to_vec();
                        temp_path.push(step);
                        total_next.push(temp_path);
                    }
                }
            }

            stack = total_next;
        }

        let mut result: Vec<Vec<String>> = vec![];
        for path in stack {
            if let Some(last_word) = path.last() {
                if last_word.eq(&end_word) {
                    result.push(path);
                }
            }
        }

        result
    }

    fn execute_bfs_with_results(
        from_word: &String,
        to_word: &String,
        words: &HashSet<String>,
    ) -> (Vec<String>, bool) {
        let next = Self::get_nearest_words(from_word, words);

        for word in &next {
            if word.eq(to_word) {
                return (next, true);
            }
        }

        (next, false)
    }

    fn get_nearest_words(word: &String, words: &HashSet<String>) -> Vec<String> {
        let mut result = vec![];

        let chars: Vec<char> = word.chars().collect();
        for i in 0..chars.len() {
            for ch in 'a' as u8..'z' as u8 {
                if chars[i] != ch as char {
                    let mut new_chars = chars.clone();
                    new_chars[i] = ch as char;
                    let s: String = new_chars.into_iter().collect();
                    if words.contains(&s) {
                        result.push(s);
                    }
                }
            }
        }

        result
    }
}
