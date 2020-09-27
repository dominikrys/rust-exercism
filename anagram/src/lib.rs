use std::collections::HashSet;
use std::iter::FromIterator;

pub fn sort_string(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    String::from_iter(chars)
}

pub fn is_same_string_case_insensitive(word1: &str, word2: &str) -> bool {
    return &word1.to_lowercase() == &word2.to_lowercase();
}

pub fn is_anagram(word1: &str, word2: &str) -> bool {
    return sort_string(&word1.to_lowercase()) == sort_string(&word2.to_lowercase());
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let anagrams: Vec<&str> = possible_anagrams
        .iter()
        .cloned()
        .filter(|&i| !is_same_string_case_insensitive(&i, &word) && is_anagram(&i, &word))
        .collect();

    HashSet::from_iter(anagrams)
}
