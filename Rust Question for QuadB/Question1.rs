// Implement a function that checks whether a given string is a palindrome or not.
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
                           .filter(|c| c.is_alphanumeric())
                           .map(|c| c.to_lowercase().next().unwrap())
                           .collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    let test_str = "aabbaa";
    if is_palindrome(test_str) {
        println!("'{}' is a palindrome.", test_str);
    } else {
        println!("'{}' is not a palindrome.", test_str);
    }
}
