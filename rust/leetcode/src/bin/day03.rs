// Day 03: Maximum Subarray
// Incomplete, but definitely learned a lot.

struct Solution{}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        return match nums.as_slice() {
            [num] => *num,
            _ => Self::find_max_sub_array(nums),
        }
    }

    fn find_max_sub_array(nums: Vec<i32>) -> i32 {
        let mut cur_max: i32 = i32::min_value();
        let mut tmp_max: i32 = i32::min_value();

        for n in nums.iter() {
            if n > &tmp_max {
                tmp_max = *n;
            }
            else {
                tmp_max += *n;
            }

            if tmp_max > cur_max {
                cur_max = tmp_max;
            }
        }

        let total_sum: i32 = nums.iter().sum();

        return if cur_max > total_sum { cur_max } else { total_sum };
    }
}

fn main() {
    let nums: Vec<i32> = vec![-2, 1];
    println!("Hello there, Day03!, {:?}", &nums[1..=1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        let mut nums: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);

        nums = vec![-2, 1];
        assert_eq!(Solution::max_sub_array(nums), 1);

        nums = vec![1, 2];
        assert_eq!(Solution::max_sub_array(nums), 3);

        nums = vec![-1];
        assert_eq!(Solution::max_sub_array(nums), -1);

        nums = vec![-2, -1];
        assert_eq!(Solution::max_sub_array(nums), -1);

        // TODO FIXME I'm failing here.
        nums = vec![8, -19, 5, -4, 20];
        assert_eq!(Solution::max_sub_array(nums), 21);

    }
}
