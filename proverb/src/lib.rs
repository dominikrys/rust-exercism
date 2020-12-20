use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|i_and_next| {
            format!(
                "For want of a {0} the {1} was lost.\n",
                i_and_next[0], i_and_next[1]
            )
        })
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .collect()
}
