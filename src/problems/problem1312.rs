// Title: Minimum Insertion Steps to Make a String Palindrome
// Difficulty: Hard
// Category: Longest Common Subsequence
// Link: https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/
// Last Executed: Thu, 2023-08-03 14:57:43 AEST

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let n = s.len();
        let s_chars: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![0; n]; n]; // 2D DP table to store the minimum insertions required

        // Loop through all possible lengths of substrings
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1; // Ending index of the current substring

                // If the characters at both ends of the substring are the same, no insertion is needed
                if s_chars[i] == s_chars[j] {
                    dp[i][j] = dp[i + 1][j - 1];
                } else {
                    // If characters are different, choose the minimum between inserting at the beginning
                    // of the substring or inserting at the end of the substring and incrementing the count
                    dp[i][j] = dp[i][j - 1].min(dp[i + 1][j]) + 1;
                }
            }
        }

        dp[0][n - 1] // The result is the minimum insertions required for the entire string
    }
}

pub fn solve() {
    // Input: s = "mbadm"
    let s = "mbadm".to_string();
    let result = Solution::min_insertions(s);

    println!("The minimum number of steps is: {}", result);
}
