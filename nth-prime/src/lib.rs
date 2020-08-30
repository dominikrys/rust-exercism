pub fn nth(n: u32) -> u32 {
    let n = n as usize;

    // Initialise with capacity so the vector is not resized multiple times
    let mut known_primes = Vec::with_capacity(n + 1);
    known_primes.push(2);

    let mut candidate = 3;

    while known_primes.len() <= n {
        if known_primes
            .iter()
            // .all() will test if the given predicate is true for all elements (i.e. if our candidate is divisible by any other primes)
            .all(|prime| candidate % prime != 0)
        {
            known_primes.push(candidate);
        }

        candidate += 2
    }

    known_primes[n]
}
