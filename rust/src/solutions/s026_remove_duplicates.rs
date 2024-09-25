use super::{Solution};

impl Solution {

  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    let nums_length = nums.len();
    if nums_length <= 1 { return nums_length as i32; }
  
    let mut last_sorted_position = 0;

    for i in 1..nums_length {

      if nums[last_sorted_position] != nums[i] {
        last_sorted_position += 1;
        nums[last_sorted_position] = nums[i];
      }
    }

    nums.truncate(last_sorted_position + 1);
    nums.len() as i32
  }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {
        assert_eq!(Solution::remove_duplicates(&mut vec![]), 0);
        let mut vec1 = vec![1, 1, 1, 1, 3];
        assert_eq!(Solution::remove_duplicates(&mut vec1), 2);
        assert_eq!(vec1, vec![1, 3]);
        let mut vec2 = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut vec2), 2);
        assert_eq!(vec2, vec![1, 2]);
    }
}