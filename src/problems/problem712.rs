// Title: Minimum ASCII Delete Sum for Two Strings
// Difficulty : Medium
// Category: On String
// Link: https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/
// Last Executed: Thu, 2023-08-03 14:57:53 AEST

struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        // Create a fixed-size array `prev_row` to store the minimum ASCII delete sum for each position in the previous row.
        let mut prev_row = [0; 1001]; // Assuming the maximum length of s2 is 1000

        // Convert input strings `s1` and `s2` into vectors of characters for easier indexing and comparison.
        let s1 = s1.chars().collect::<Vec<_>>();
        let s2 = s2.chars().collect::<Vec<_>>();

        // Fill the first row of `prev_row` with the sum of ASCII values of characters in the prefix of `s2`.
        for j in 1..=s2.len() {
            prev_row[j] = prev_row[j - 1] + s2[j - 1] as i32;
        }

        // Iterate over each character in `s1` to calculate the minimum ASCII delete sum for each position in the current row.
        for i in 1..=s1.len() {
            // Create a new fixed-size array `curr_row` to store the minimum ASCII delete sum for each position in the current row.
            let mut curr_row = [0; 1001];
            // Calculate the first value of `curr_row`, which is the sum of ASCII values of characters in the prefix of `s1`.
            curr_row[0] = prev_row[0] + s1[i - 1] as i32;

            // Iterate over each character in `s2`.
            for j in 1..=s2.len() {
                // Check if the characters at the current positions in `s1` and `s2` are the same.
                if s1[i - 1] == s2[j - 1] {
                    // If the characters are the same, no deletion is required, so set `curr_row[j]` to `prev_row[j-1]`.
                    curr_row[j] = prev_row[j - 1];
                } else {
                    // If the characters are different, we need to consider either deleting `s1[i-1]` or `s2[j-1]`.
                    // Set `curr_row[j]` to the minimum of two values:
                    // 1. `prev_row[j] + s1[i-1] as i32`, which represents the ASCII value of `s1[i-1]` plus the minimum ASCII delete sum up to the previous position in `s2`.
                    // 2. `curr_row[j-1] + s2[j-1] as i32`, which represents the ASCII value of `s2[j-1]` plus the minimum ASCII delete sum up to the current position in `s2`.
                    curr_row[j] = std::cmp::min(prev_row[j] + s1[i - 1] as i32, curr_row[j - 1] + s2[j - 1] as i32);
                }
            }

            // Update `prev_row` with `curr_row` for the next iteration.
            prev_row = curr_row;
        }

        // The last value in `prev_row` represents the minimum ASCII delete sum for the complete strings `s1` and `s2`.
        prev_row[s2.len()]
    }
}

pub fn solve() {
    // Input: s1 = "sea", s2 = "eat"
    let s1 = "sea".to_string();
    let s2 = "eat".to_string();
    let min_delete_sum = Solution::minimum_delete_sum(s1, s2);
    println!("Minimum ASCII Delete Sum: {}", min_delete_sum);
}
