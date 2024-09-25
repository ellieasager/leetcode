use super::Solution;

impl Solution {
    pub fn is_palindrome_lazy(x: i32) -> bool {
        let x_str = x.to_string();
        let x_slice: &str = &x_str[..];
        let x_reversed: String = x_slice.chars().rev().collect();
        x_str == x_reversed
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { 
            return false 
        }
        if x < 10 { 
            return true 
        }
        if (x % 10) == 0 { 
            return false 
        }
        
        let mut digits: Vec<i32> = Vec::new();
        let mut input = x;
        while input > 0 {
            let digit = input % 10;
            input = input / 10;
            digits.push(digit);
        }

        let num_of_digits = digits.len();
        for i in 0..(num_of_digits/2) {
            let d1 = digits[i];
            let d2 = digits[num_of_digits - 1 - i];
            if d1 != d2 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(Solution::is_palindrome(-32), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(9), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(2222), true);
        assert_eq!(Solution::is_palindrome(11222211), true);
    }
}