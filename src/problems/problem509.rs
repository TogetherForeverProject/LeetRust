// Topic: Dynamic Programming
// Title: Fibonacci Number
// Difficulty: Easy
// Category: Fibonacci Style
// Link: https://leetcode.com/problems/fibonacci-number/
// Last Executed: Wed, 2023-08-02 11:28:06 AEST

struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        // Base cases: If n is 0 or 1, the Fibonacci number is n itself.
        if n <= 1 {
            return n;
        }

        // Initialize prev1 and prev2 to represent the first two Fibonacci numbers.
        let mut prev1 = 0;
        let mut prev2 = 1;

        // Calculate the Fibonacci number using a loop starting from the 2nd Fibonacci number (index 2).
        for _ in 2..=n {
            // Calculate the current Fibonacci number by adding the previous two values.
            let current = prev1 + prev2;
            // Update prev1 and prev2 to the current value for the next iteration.
            prev1 = prev2;
            prev2 = current;
        }

        // The result is stored in prev2, representing the Fibonacci number at index n.
        prev2
    }
}

pub fn solve() {
    // Input: n = 3
    let n = 3;
    let result = Solution::fib(n);
    println!("{}", result);
}
