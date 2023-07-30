// Topic: Dynamic Programming
// Title: N-th Tribonacci Number
// Difficulty: Easy
// Category: Fibonacci Style
// Link: https://leetcode.com/problems/n-th-tribonacci-number/
// Last Executed: Mon, 2023-07-31 05:12:20 AEST

struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        // Base cases: T(0) = 0, T(1) = 1, T(2) = 1
        if n == 0 {
            return 0;
        } else if n <= 2 {
            return 1;
        }

        // Initialize prev1, prev2, and prev3 to represent the first three Tribonacci numbers.
        let mut prev1 = 0;
        let mut prev2 = 1;
        let mut prev3 = 1;

        // Calculate the Tribonacci number using a loop starting from the 3rd Tribonacci number (index 3).
        for _ in 3..=n {
            // Calculate the current Tribonacci number by adding the previous three values.
            let current = prev1 + prev2 + prev3;
            // Update prev1, prev2, and prev3 to the current value for the next iteration.
            prev1 = prev2;
            prev2 = prev3;
            prev3 = current;
        }

        // The result is stored in prev3, representing the Tribonacci number at index n.
        prev3
    }
}

pub fn solve() {
    // Input: n = 25
    let n = 25;
    let tribonacci_number = Solution::tribonacci(n);

    println!("Tribonacci number at index {}: {}", n, tribonacci_number);
}
