fn main() {
    const L: usize = 1_000_000;
    // Alternate form of sieve -- just gather bool rather than count like we
    // needed a few problems ago.
    let mut sieve: Vec<bool> = vec![true; L];
    let mut primes: Vec<u64> = Vec::new();

    for i in 2..L {
        if sieve[i] {
            primes.push(i as u64);
            let mut j = i * i;
            while j < L {
                sieve[j] = false;
                j += i;
            }
        }
    }

    let mut prefix_sums = vec![0u64];
    let mut current_sum = 0u64;
    for &p in &primes {
        current_sum += p as u64;
        if current_sum >= L as u64 {
            break;
        }
        prefix_sums.push(current_sum);
    }

    println!(
        "Total primes: {}, Useful prefix sums: {}",
        primes.len(),
        prefix_sums.len()
    );
    let mut max: u64 = 0;
    for i in 0..prefix_sums.len() {
        for j in i + 1..prefix_sums.len() {
            let diff = prefix_sums[j] - prefix_sums[i];
            if diff > max && primes.contains(&diff) {
                max = diff;
            }
        }
    }
    println!("max: {}", max);
}
