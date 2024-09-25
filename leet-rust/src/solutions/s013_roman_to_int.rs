use std::collections::HashMap;

use super::Solution;

impl Solution {

    pub fn roman_to_int(s: String) -> i32 {
        
        let mut values: HashMap<char, i32> = HashMap::new();
        values.insert('I', 1);
        values.insert('V', 5);
        values.insert('X', 10);
        values.insert('L', 50);
        values.insert('C', 100);
        values.insert('D', 500);
        values.insert('M', 1000);


        let characters: Vec<char> = s.chars().collect();
        if s.len() == 1 {
            return *values.get(&characters[0]).unwrap();
        }

        let mut sum = 0;
        let mut i = 0;

        while i < characters.len() {
            let cur_num = values.get(&characters[i]).unwrap();
            let next_num = if i < characters.len() - 1 {
                values.get(&characters[i+1]).unwrap()
            } else {
                &0
            };

            if cur_num >= next_num {
                sum += cur_num;
                i += 1;

            } else { // cur_num < next_num
                sum = sum + next_num - cur_num;
                i += 2;
            }
        }

        sum


    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
    }
}