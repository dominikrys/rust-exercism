use std::collections::HashMap;

pub fn get_letter_frequencies(input: &str) -> HashMap<char, usize> {
    let mut freqs = HashMap::<char, usize>::new();
    input.chars().for_each(|c| {
        if c.is_alphabetic() {
            *freqs.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        }
    });

    freqs
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut freqs = HashMap::<char, usize>::new();

    let input = input.join("");
    if input.is_empty() {
        return freqs;
    }

    let mut thread_pool = Vec::with_capacity(worker_count);
    let chunk_size = input.len() / worker_count + 1;

    for _ in 0..worker_count {
        let chunk = input.chars().by_ref().take(chunk_size).collect::<String>();

        let thread = std::thread::spawn(move || get_letter_frequencies(&chunk));

        thread_pool.push(thread);
    }

    for thread in thread_pool {
        for (key, val) in thread.join().unwrap().iter() {
            *freqs.entry(*key).or_insert(0) += val;
        }
    }

    freqs
}
