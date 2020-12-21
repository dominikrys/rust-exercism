pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_stack: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => bracket_stack.push(c),
            ')' => if bracket_stack.pop() != Some('(') { return false },
            ']' => if bracket_stack.pop() != Some('[') { return false },
            '}' => if bracket_stack.pop() != Some('{') { return false },
            _ => ()
        }
    }

    bracket_stack.is_empty()
}
