use super::Solution;

impl Solution {

    pub fn is_valid(s: String) -> bool {

        let mut stack: Vec<char> = Vec::new();

        for current_char in s.chars() {
            let last_paren = stack.last();

            match last_paren {
                Some(&paren_char) => {
                    if Solution::is_pair(paren_char, current_char) {
                        stack.pop();
                        continue;
                    }
                }
                None => {},
            };
            stack.push(current_char);
        }
        stack.is_empty()
    }

    #[inline]
    fn is_pair(char1: char, char2: char) -> bool {
        char1 == '(' && char2 == ')' ||
        char1 == '{' && char2 == '}' ||
        char1 == '[' && char2 == ']'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests {
        use super::*;
    
        #[test]
        fn test_20() {
            assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
            assert_eq!(Solution::is_valid("({}]".to_string()), false);
        }
    }
}