use std::collections::HashMap;

pub fn count_letters(strings: Vec<String>) -> HashMap<char, usize> {
    println!("{:?}" , strings);

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
        .map(|strings| std::thread::spawn(move || count_letters(strings)))
        .map(|thread| thread.join().unwrap())
        .flat_map(|freqs| freqs.into_iter())
        .fold(HashMap::new(), |mut acc, (ch, freq)| {
            *acc.entry(ch).or_insert(0) += freq;
            acc
        })
}
