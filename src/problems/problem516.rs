// Title: Longest Palindromic Subsequence
// Difficulty: Medium
// Category: On String
// Link: https://leetcode.com/problems/longest-palindromic-subsequence/
// Last Executed: Wed, 2023-08-02 01:23:49 AEST

struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut dp = vec![vec![0; n]; n];

        // Base case: single characters are palindromes by themselves
        for i in 0..n {
            dp[i][i] = 1;
        }

        // Calculate the length of the longest palindromic subsequence
        // for subsequences of length 2 and above
        for len in 2..=n {
            for i in 0..n - len + 1 {
                let j = i + len - 1;
                if chars[i] == chars[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
                }
            }
        }

        dp[0][n - 1]
    }
}

pub fn solve() {
    // Input: s = "bbbab"
    let s = "bbbab".to_string();
    let longest_palindrome_len = Solution::longest_palindrome_subseq(s);
    println!("Longest Palindromic Subsequence Length: {}", longest_palindrome_len);
}
