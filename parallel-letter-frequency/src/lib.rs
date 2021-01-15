use std::collections::HashMap;

pub fn get_letter_freqs(input: &std::vec::Vec<std::string::String>) -> HashMap<char, usize> {
    input
        .iter()
        .flat_map(|s| s.chars().filter(|ch| ch.is_alphabetic()))
        .map(|ch| ch.to_ascii_lowercase())
        .fold(HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut freqs = HashMap::<char, usize>::new();

    let chunk_size = (input.len() / worker_count).max(1);
    let input: Vec<_> = input.iter().map(|s| s.to_string()).collect();

    let thread_pool: Vec<_> = input
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .map(|chunk| std::thread::spawn(move || get_letter_freqs(&chunk)))
        .collect();

    for thread in thread_pool {
        for (ch, freq) in thread.join().unwrap().into_iter() {
            *freqs.entry(ch).or_insert(0) += freq;
        }
    }
    freqs
}
