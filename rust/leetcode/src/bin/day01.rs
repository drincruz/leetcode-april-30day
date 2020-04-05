// Day 01

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut numbers = HashMap::new();
        for n in nums {
            let counter = numbers.entry(n).or_insert(0);
            *counter += 1;
        }

        for (number, count) in numbers.iter() {
            if *count == 1 {
                return *number;
            }
        }

        return -1;
    }
}

fn main() {
    // First example
    let nums: Vec<i32> = vec![4, 1, 2, 1, 2];
    let single_number = Solution::single_number(nums);
    println!("Single number: {}", single_number);
}
