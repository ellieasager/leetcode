pub mod solutions;

fn main() {
    println!("Hello, world!");
    let result = is_divisible_by(3, 1);
    println!("result: {}", result);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_divisible() {
        assert!(is_divisible_by(0, 1));
    }

    #[test]
    fn test_not_divisible() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert!(!is_divisible_by(1, 0));
    }
}
