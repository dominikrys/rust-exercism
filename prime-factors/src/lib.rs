pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut candidate = 1;

    while n > 1 {
        candidate = candidate + 1;

        while n % candidate == 0 {
            n /= candidate;
            factors.push(candidate);
        }
    }

    factors
}
