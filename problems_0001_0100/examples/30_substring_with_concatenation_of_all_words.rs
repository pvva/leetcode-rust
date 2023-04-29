/*
https://leetcode.com/problems/substring-with-concatenation-of-all-words/

You are given a string s and an array of strings words. All the strings of words are of the same length.
A concatenated substring in s is a substring that contains all the strings of any permutation of words concatenated.

For example, if words = ["ab","cd","ef"], then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" are
all concatenated strings. "acdbef" is not a concatenated substring because it is not the concatenation of any
permutation of words. Return the starting indices of all the concatenated substrings in s.
You can return the answer in any order.

Example 1:

Input: s = "barfoothefoobarman", words = ["foo","bar"]
Output: [0,9]
Explanation: Since words.length == 2 and words[i].length == 3, the concatenated substring has to be of length 6.
The substring starting at 0 is "barfoo". It is the concatenation of ["bar","foo"] which is a permutation of words.
The substring starting at 9 is "foobar". It is the concatenation of ["foo","bar"] which is a permutation of words.
The output order does not matter. Returning [9,0] is fine too.
Example 2:

Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
Output: []
Explanation: Since words.length == 4 and words[i].length == 4, the concatenated substring has to be of length 16.
There is no substring of length 16 is s that is equal to the concatenation of any permutation of words.
We return an empty array.
Example 3:

Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
Output: [6,9,12]
Explanation: Since words.length == 3 and words[i].length == 3, the concatenated substring has to be of length 9.
The substring starting at 6 is "foobarthe". It is the concatenation of ["foo","bar","the"] which is a permutation of words.
The substring starting at 9 is "barthefoo". It is the concatenation of ["bar","the","foo"] which is a permutation of words.
The substring starting at 12 is "thefoobar". It is the concatenation of ["the","foo","bar"] which is a permutation of words.

Constraints:

1 <= s.length <= 104
1 <= words.length <= 5000
1 <= words[i].length <= 30
s and words[i] consist of lowercase English letters.
*/

fn main() {}

struct Solution;

// Solution goes bellow.

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        let mut w_len: usize = 0;
        let words_cnt = words.len();
        let mut words_counts: HashMap<String, usize> = HashMap::new();
        for w in words {
            w_len = w.len();
            let cnt = match words_counts.get(&w) {
                None => { 0 }
                Some(c) => { *c }
            } + 1;
            words_counts.insert(w, cnt);
        }

        let mut tracked_words_count: HashMap<String, usize> = HashMap::new();
        let mut idx: usize = 0;
        let chars: Vec<u8> = s.bytes().collect();
        while idx + w_len * words_cnt <= s.len() {
            let current_word = std::str::from_utf8(&chars[idx..idx + w_len]).unwrap().to_string();
            if words_counts.contains_key(&current_word) {
                tracked_words_count.insert(current_word, 1);
                let mut t_cnt: usize = 1;
                let mut in_sequence = true;
                while t_cnt < words_cnt && in_sequence {
                    in_sequence = false;
                    let s = idx + t_cnt * w_len;
                    let next_word = std::str::from_utf8(&chars[s..s + w_len]).unwrap().to_string();
                    if let Some(word_cnt_limit) = words_counts.get(&next_word) {
                        let tracked_cnt = match tracked_words_count.get(&next_word) {
                            None => { 0 }
                            Some(c) => { *c }
                        } + 1;
                        if tracked_cnt <= *word_cnt_limit {
                            in_sequence = true;
                            tracked_words_count.insert(next_word, tracked_cnt);
                            t_cnt += 1;
                        }
                    }
                }
                if t_cnt == words_cnt {
                    result.push(idx as i32);
                }
                tracked_words_count.clear();
            }
            idx += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string_vec(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_find_substring_1(){
        assert_eq!(vec![0, 9],
                   Solution::find_substring("barfoothefoobarman".to_string(),
                                            to_string_vec(vec!["foo", "bar"])));
    }

    #[test]
    fn test_find_substring_2(){
        assert_eq!(vec![] as Vec<i32>,
                   Solution::find_substring("wordgoodgoodgoodbestword".to_string(),
                                            to_string_vec(vec!["word","good","best","word"])));
    }

    #[test]
    fn test_find_substring_3(){
        assert_eq!(vec![6, 9, 12],
                   Solution::find_substring("barfoofoobarthefoobarman".to_string(),
                                            to_string_vec(vec!["bar","foo","the"])));
    }

    #[test]
    fn test_find_substring_4(){
        assert_eq!(vec![8],
                   Solution::find_substring("wordgoodgoodgoodbestword".to_string(),
                                            to_string_vec(vec!["word","good","best","good"])));
    }

    #[test]
    fn test_find_substring_5(){
        assert_eq!(vec![] as Vec<i32>,
                   Solution::find_substring("a".to_string(),
                                            to_string_vec(vec!["a", "a"])));
    }

    #[test]
    fn test_find_substring_6(){
        assert_eq!(vec![13] as Vec<i32>,
                   Solution::find_substring("lingmindraboofooowingdingbarrwingmonkeypoundcake".to_string(),
                                            to_string_vec(vec!["fooo","barr","wing","ding","wing"])));
    }
}
