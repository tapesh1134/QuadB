// Implement a function that finds the longest common prefix of a given set of strings.
fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = strs[0].clone();

    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }

    prefix
}

fn main() {
    let strings = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];

    let prefix = longest_common_prefix(&strings);
    println!("The longest common prefix is '{}'", prefix);
}
