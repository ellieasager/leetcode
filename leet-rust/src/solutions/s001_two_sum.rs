
use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut num_map: HashMap<i32, i32> = HashMap::new();
        for (i, &value) in nums.iter().enumerate() {
            let complement = target - value;
            if let Some((_, &v)) = num_map.get_key_value(&complement) {
                return vec![v, i as i32];
            }
            num_map.insert(value, i as i32);
        }
        vec![]
    }

    pub fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}