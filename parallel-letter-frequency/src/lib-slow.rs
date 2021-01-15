use std::collections::HashMap;

// Slower solution than sequential in all cases, but very elegant

pub fn count_letters(strings: Vec<String>) -> HashMap<char, usize> {
    strings
        .iter()
        .flat_map(|s| s.chars().filter(|ch| ch.is_alphabetic()))
        .fold(HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    input
        .chunks(input.len() / worker_count + 1)
        .map(|chunk| {
            chunk
                .iter()
                .map(|s| s.to_string().to_lowercase())
                .collect::<Vec<String>>()
        })
        .map(|chunks| std::thread::spawn(move || count_letters(chunks)))
        .map(|thread| thread.join().unwrap())
        .flat_map(|freqs| freqs.into_iter())
        .fold(HashMap::new(), |mut acc, (ch, freq)| {
            *acc.entry(ch).or_insert(0) += freq;
            acc
        })
}
