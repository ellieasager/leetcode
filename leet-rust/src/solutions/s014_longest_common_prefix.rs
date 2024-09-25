use super::Solution;
use std::str::Chars;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        if strs.len() == 0 || strs[0].len() == 0 {
            return String::default();
        }
        if strs.len() == 1 {
            return strs[0].to_owned();
        }
        
        let mut prefix = String::new();
        let mut sorted_strs = strs.to_vec();
        sorted_strs.sort_by(|a, b| a.len().cmp(&b.len()));
        let sorted_words: Vec<Chars> = sorted_strs.iter().map(|word| word.chars()).collect();

        if let Some((first_str, other_strs)) = sorted_words.split_first() {
            let first_word = first_str.to_owned();
            let mut other_words = other_strs.to_owned();

            for current_ch in first_word {                

                for current_word in other_words.iter_mut() {
                    let mut ch = current_word.next();
                    if current_ch != ch.unwrap() {
                        return prefix;
                    }
                    ch.take();
                }
                prefix.push(current_ch);
            }
        }

        prefix

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "cir".to_string(),
                "car".to_string()
            ]),
            "c"
        );        
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
    }
}