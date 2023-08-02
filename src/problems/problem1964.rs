// Title: Find the Longest Valid Obstacle Course at Each Position
// Difficulty: Hard
// Category: On String
// Link: https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position/
// Last Executed: Wed, 2023-08-02 11:28:19 AEST

struct Solution;

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let n = obstacles.len();
        let mut longest_obstacle_course = vec![0; n];
        let mut lis = vec![0; n];
        let mut lis_length = 0;

        for i in 0..n {
            let mut left = 0;
            let mut right = lis_length;
            while left < right {
                let middle = (left + right) / 2;
                if lis[middle] <= obstacles[i] {
                    left = middle + 1;
                } else {
                    right = middle;
                }
            }

            if left >= lis_length || lis[left] > obstacles[i] {
                lis[left] = obstacles[i];
            }

            if left == lis_length {
                lis_length += 1;
            }

            longest_obstacle_course[i] = left as i32 + 1;
        }

        longest_obstacle_course
    }
}

pub fn solve() {
    // Input: obstacles = [1,3,2,4,5,3,2]
    let obstacles = vec![1, 3, 2, 4, 5, 3, 2];
    let ans = Solution::longest_obstacle_course_at_each_position(obstacles);
    println!("{:?}", ans); // Output: [1, 2, 2, 3, 4, 4, 4]
}
