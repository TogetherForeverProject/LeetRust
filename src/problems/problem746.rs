// Topic: Dynamic Programming
// Title: Min Cost Climbing Stairs
// Difficulty: Easy
// Category: Fibonacci Style
// Link: https://leetcode.com/problems/min-cost-climbing-stairs/
// Last Executed: Thu, 2023-08-03 14:57:56 AEST

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut first = cost[0];
        let mut second = cost[1];

        for i in 2..cost.len() {
            let temp = second;
            second = first.min(second) + cost[i];
            first = temp;
        }

        first.min(second)
    }
}

pub fn solve() {
    // Input: n = [1,100,1,1,1,100,1,1,100,1]
    let cost = vec![1,100,1,1,1,100,1,1,100,1];
    let min_cost = Solution::min_cost_climbing_stairs(cost);

    println!("Minimum cost to reach the top: {}", min_cost);
}
