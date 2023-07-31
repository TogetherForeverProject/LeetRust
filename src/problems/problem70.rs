// Topic: Dynamic Programming
// Title: Climbing Stairs
// Difficulty: Easy
// Category: Fibonacci Style
// Link: https://leetcode.com/problems/climbing-stairs/
// Last Executed: Mon, 2023-07-31 19:50:43 AEST

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // Base cases: If n is 1 or 2, there are n distinct ways to climb the stairs.
        if n <= 2 {
            return n;
        }

        // Initialize prev1 and prev2 to represent the number of ways to reach the first and second steps.
        let (mut prev1, mut prev2) = (1, 2);

        // Calculate the number of ways to reach the nth step using a loop.
        for _ in 3..=n {
            // Calculate the current number of ways by adding the previous two values.
            let current = prev1 + prev2;
            // Update prev1 and prev2 to the current value for the next iteration.
            prev1 = prev2;
            prev2 = current;
        }

        // The result is stored in prev2, representing the number of distinct ways to reach the top of the staircase with n steps.
        prev2
    }
}

pub fn solve() {
    // Input: n = 2
    let n = 2;
    let ways = Solution::climb_stairs(n);
    println!("{}", ways);
}
