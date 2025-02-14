pub fn reverse_string(input: &str) -> String{
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("Hello"), "olleH");
    }
}

