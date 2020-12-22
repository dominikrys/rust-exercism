#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn list_contains<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    second.is_empty() || first.windows(second.len()).any(|w| w == second)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use std::cmp::Ordering::*;

    match first_list.len().cmp(&second_list.len()) {
        Equal if first_list == second_list => Comparison::Equal,
        Less if list_contains(second_list, first_list) => Comparison::Sublist,
        Greater if list_contains(first_list, second_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
