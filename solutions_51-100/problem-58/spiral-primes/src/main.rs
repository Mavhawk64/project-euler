use is_prime::is_prime;

fn main() {
    // We start at side length 1:
    // diagonals = {1}, prime_count = 0, diag_count = 1
    // this is essentially just an optimized, looped version of problem 28
    // but counts primes instead of sums!
    let mut side: u64 = 1;
    let mut diag_count: u64 = 1;
    let mut prime_count: u64 = 0;

    // This loop is just a less-visualized form of the
    // problem 28: number-spiral-diagonals.
    // I've taken the logic from that and extracted it to just counting primes as I go along.
    loop {
        // I've found (and you'll find through the pictures)
        // that you just add 2 to the side to get the next odd ring.
        side += 2;
        // after that, you add 4 corners at each iteration!
        diag_count += 4;

        // this is where the black magic happens
        // see my problem 28 for a bit of a better instruction.
        // this is the one corner that IS NEVER PRIME:
        // (2n+1)**2 -- odd squares
        let s2 = side * side;
        let step = side - 1;

        // These are the 3 possible corner primes!
        let c1 = s2 - step;
        let c2 = s2 - 2 * step;
        let c3 = s2 - 3 * step;

        prime_count += is_prime(&c1.to_string()) as u64;
        prime_count += is_prime(&c2.to_string()) as u64;
        prime_count += is_prime(&c3.to_string()) as u64;

        let ratio = prime_count as f64 / diag_count as f64;

        // just go until we've hit below 10%! (this occurs at 26,241)

        if ratio < 0.10 {
            println!(
                "First side length where ratio < 10%: side = {}, primes = {}/{}, ratio = {}",
                side, prime_count, diag_count, ratio
            );
            break;
        }
    }
}
