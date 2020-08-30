pub fn reverse(input: &str) -> String {
    // rev() returns an iterator, so it needs to be transformed into a collection using .collect()
    input.chars().rev().collect::<String>()
}
