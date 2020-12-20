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

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use std::cmp::Ordering::*;

    match _first_list.len().cmp(&_second_list.len()) {
        Equal if _first_list == _second_list => Comparison::Equal,
        Less if list_contains(_second_list, _first_list) => Comparison::Sublist,
        Greater if list_contains(_first_list, _second_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
