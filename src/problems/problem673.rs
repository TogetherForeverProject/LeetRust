// Title: Number of Longest Increasing Subsequence
// Difficulty: Medium
// Category: Longest Increasing Subsequence
// Link: https://leetcode.com/problems/number-of-longest-increasing-subsequence/
// Last Executed: Thu, 2023-08-03 14:57:52 AEST

struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut lengths = vec![1; n];
        let mut counts = vec![1; n];
        let mut max_length = 1;
        let mut result = 0;

        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    if lengths[j] + 1 > lengths[i] {
                        lengths[i] = lengths[j] + 1;
                        counts[i] = counts[j];
                    } else if lengths[j] + 1 == lengths[i] {
                        counts[i] += counts[j];
                    }
                }
            }

            if lengths[i] > max_length {
                max_length = lengths[i];
                result = counts[i];
            } else if lengths[i] == max_length {
                result += counts[i];
            }
        }

        result
    }
}

pub fn solve() {
    // Input: nums = [1,3,5,4,7]
    let nums = vec![1, 3, 5, 4, 7];
    let result = Solution::find_number_of_lis(nums);
    println!("Number of longest increasing subsequences: {}", result);
}
