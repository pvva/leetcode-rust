/*
https://leetcode.com/problems/word-ladder-ii/

Given two words (beginWord and endWord), and a dictionary's word list, find all shortest transformation
sequence(s) from beginWord to endWord, such that:

Only one letter can be changed at a time
Each transformed word must exist in the word list. Note that beginWord is not a transformed word.
Note:

Return an empty list if there is no such transformation sequence.
All words have the same length.
All words contain only lowercase alphabetic characters.
You may assume no duplicates in the word list.
You may assume beginWord and endWord are non-empty and are not the same.
*/

fn to_strings(input: Vec<&str>) -> Vec<String> {
    let mut result = vec![];

    for str in input {
        result.push(str.to_string());
    }

    result
}

fn main() {
    println!(
        "{:?}",
        Solution::find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            to_strings(vec!["hot", "dot", "dog", "lot", "log"])
        )
    );
    println!(
        "{:?}",
        Solution::find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            to_strings(vec!["hot", "dot", "dog", "lot", "log", "cog"])
        )
    );
    println!(
        "{:?}",
        Solution::find_ladders(
            "qa".to_string(),
            "sq".to_string(),
            to_strings(vec![
                "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm", "le",
                "av", "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo", "ow", "sn",
                "ya", "cr", "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur", "rh", "sr", "tc",
                "lt", "lo", "as", "fr", "nb", "yb", "if", "pb", "ge", "th", "pm", "rb", "sh", "co",
                "ga", "li", "ha", "hz", "no", "bi", "di", "hi", "qa", "pi", "os", "uh", "wm", "an",
                "me", "mo", "na", "la", "st", "er", "sc", "ne", "mn", "mi", "am", "ex", "pt", "io",
                "be", "fm", "ta", "tb", "ni", "mr", "pa", "he", "lr", "sq", "ye"
            ])
        )
    );
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
