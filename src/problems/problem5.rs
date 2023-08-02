// Title: Longest Palindromic Substring
// Difficulty: Medium
// Category: On String
// Link: https://leetcode.com/problems/longest-palindromic-substring/
// Last Executed: Wed, 2023-08-02 11:28:08 AEST

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // Convert the input string to bytes for efficient indexing
        let s = s.as_bytes();
        let n = s.len();
        let mut start = 0;
        let mut end = 0;

        for i in 0..n {
            // Expand around the current character as center for odd-length palindromes
            let mut left = i;
            let mut right = i;

            // Check for duplicates to handle even-length palindromes
            while right + 1 < n && s[right + 1] == s[left] {
                right += 1;
            }

            // Expand the palindrome around the center
            while left > 0 && right + 1 < n && s[left - 1] == s[right + 1] {
                left -= 1;
                right += 1;
            }

            // Update the longest palindrome's start and end indices
            if right - left > end - start {
                start = left;
                end = right;
            }
        }

        // Convert the longest palindrome back to a String and return it
        String::from_utf8(s[start..=end].to_vec()).unwrap()
    }
}

pub fn solve() {
    // Input: s = "babad"
    let s = "babad".to_string();
    let longest_palindrome = Solution::longest_palindrome(s);
    println!("Longest palindromic substring: {}", longest_palindrome);
}
