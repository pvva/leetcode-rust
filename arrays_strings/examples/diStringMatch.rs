/*
Given a string S that only contains "I" (increase) or "D" (decrease), let N = S.length.

Return any permutation A of [0, 1, ..., N] such that for all i = 0, ..., N-1:

If S[i] == "I", then A[i] < A[i+1]
If S[i] == "D", then A[i] > A[i+1]


Example 1:

Input: "IDID"
Output: [0,4,1,3,2]
Example 2:

Input: "III"
Output: [0,1,2,3]
*/

fn main() {
    println!("{:?}", di_string_match("IDID".to_string()));
    println!("{:?}", di_string_match("III".to_string()));
}

fn di_string_match(s: String) -> Vec<i32> {
    let mut d = s.len();
    let mut i = 0;
    let mut v = vec![0; d + 1];
    for (idx, ch) in s.chars().enumerate() {
        if ch == 'I' {
            v[idx] = i;
            i += 1;
        } else {
            v[idx] = d as i32;
            d -= 1;
        }
    }
    v[s.len()] = i;
    v
}
