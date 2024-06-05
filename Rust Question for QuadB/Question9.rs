// Reverse a string in Rust
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let original = "Hello";
    let reversed = reverse_string(&original);
    println!("Original: '{}'", original);
    println!("Reversed: '{}'", reversed);
}
