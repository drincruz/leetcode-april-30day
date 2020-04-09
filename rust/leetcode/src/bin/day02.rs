// Day 02: Happy Number

use std::collections::HashSet;

struct Solution{}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = HashSet::new();
        let mut num = n;
        while num != 1 {
            let num_vec = Self::num_to_vec(num);
            num = Self::sum_of_squares(num_vec);
            if seen.contains(&num) {
                return false;
            }
            seen.insert(num);
        }
        return true;
    }

    pub fn num_to_vec(n: i32) -> Vec<i32> {
        let mut numbers: Vec<i32> = Vec::new();
        let mut num = n;

        while num > 0 {
            let tmp = num % 10;
            numbers.push(tmp);
            num = num / 10;
        }
        return numbers;
    }

    pub fn sum_of_squares(numbers: Vec<i32>) -> i32 {
        let mut value = 0;
        for n in numbers.iter() {
            value += *n * *n;
        }

        return value;
    }
}

fn main() {
    println!("is_happy(19) {}", Solution::is_happy(19));
    println!("is_happy(8) {}", Solution::is_happy(8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_vec() {
        let nums: Vec<i32> = vec![9, 1];
        assert_eq!(Solution::num_to_vec(19), nums);
    }

    #[test]
    fn test_sum_of_squares() {
        let nums: Vec<i32> = vec![9, 1];
        assert_eq!(Solution::sum_of_squares(nums), 82);
    }

    #[test]
    fn test_is_happy() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(8), false);
    }
}
