// Title: Word Break
// Difficulty: Medium
// Category: On String
// Link: https://leetcode.com/problems/word-break/
// Last Executed: Wed, 2023-08-02 11:28:02 AEST

struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // Create a set of word_dict for faster lookup
        let word_dict_set: HashSet<String> = word_dict.into_iter().collect();

        // Sort and deduplicate the word lengths to prioritize longer words for matching first
        let mut word_lengths: Vec<usize> = word_dict_set.iter().map(|word| word.len()).collect();
        word_lengths.sort();
        word_lengths.dedup();

        // Initialize a HashSet to keep track of indexes already searched
        let mut visited: HashSet<usize> = HashSet::new();

        // Initialize a queue to perform breadth-first search
        let mut index_queue: VecDeque<usize> = VecDeque::new();
        index_queue.push_back(0);

        while let Some(current_index) = index_queue.pop_front() {
            // Skip already visited indexes
            if visited.contains(&current_index) {
                continue;
            }

            // Try matching words of different lengths starting from the current index
            for length in word_lengths.iter() {
                if current_index + length <= s.len() {
                    let word_slice = &s[current_index..current_index + length];
                    if word_dict_set.contains(word_slice) {
                        if current_index + length == s.len() {
                            return true;
                        }

                        // Add the next possible index to the queue for further exploration
                        index_queue.push_back(current_index + length);
                    }
                }
            }

            // Mark the current index as visited to avoid reprocessing it
            visited.insert(current_index);
        }

        // If no valid segmentation is found, return false
        false
    }
}

pub fn solve() {
    // Input: s = "leetcode"
    let s = "leetcode".to_string();
    let word_dict = vec![
        "leet".to_string(),
        "code".to_string(),
    ];
    let can_segment = Solution::word_break(s, word_dict);
    println!("Can be segmented: {}", can_segment);
}
