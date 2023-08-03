// Title: Longest Increasing Subsequence
// Difficulty: Medium
// Category: Longest Increasing Subsequence
// Link: https://leetcode.com/problems/longest-increasing-subsequence/
// Last Executed: Thu, 2023-08-03 14:57:46 AEST

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut piles = vec![0; n]; // Array to store the top card of each pile
        let mut len = 0; // Length of the longest increasing subsequence

        for num in nums {
            let mut left = 0;
            let mut right = len;

            while left < right {
                let mid = left + (right - left) / 2;
                if piles[mid] < num {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            if left == len {
                piles[len] = num;
                len += 1;
            } else {
                piles[left] = num;
            }
        }

        len as i32
    }
}

pub fn solve() {
    // Input: nums = [10,9,2,5,3,7,101,18]
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let result = Solution::length_of_lis(nums);
    println!("Length of the longest increasing subsequence: {}", result);
}
