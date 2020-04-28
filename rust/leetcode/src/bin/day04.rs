// Day 04: Move Zeroes
// Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.


use std::collections::HashSet;

struct Solution{}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i: usize = 0;
        let mut zeroes: HashSet<usize> = Self::zero_indices(nums.to_vec());
        let nums_len: usize = nums.len();

        while i < nums_len {
            if zeroes.contains(&i) {
                let mut j: usize = i;
                while j < (nums_len - 1) {
                    let tmp: i32 = nums[j];
                    nums[j] = nums[j + 1];
                    nums[j + 1] = tmp;
                    j += 1;
                }
                zeroes = Self::zero_indices(nums.to_vec());
            }
            i += 1;
        }
    }

    pub fn zero_indices(nums: Vec<i32>) -> HashSet<usize> {
        let mut zeros: HashSet<usize> = HashSet::new();
        for (index, value) in nums.iter().enumerate() {
            if *value == 0 {
                zeros.insert(index);
            }
        }
        return zeros;
    }
}

fn main() {
    println!("Hello there, Day04!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        nums = vec![0, 0, 1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0, 0]);
    }

    #[test]
    fn test_zero_indices() {
        let nums: Vec<i32> = vec![0, 1, 0, 3, 12];
        let zeroes: HashSet<usize> = [0, 2].iter().cloned().collect();
        assert_eq!(Solution::zero_indices(nums), zeroes);
    }
}
