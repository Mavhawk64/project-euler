use primes::is_prime;
fn main() {
    let mut mx: i32 = 0;
    let mut ma: i32 = 0;
    let mut mb: i32 = 0;
    for a in -999..1000 {
        for b in -999..1000 {
            let cnt = count_primes(a, b);
            if cnt > mx {
                mx = cnt;
                ma = a;
                mb = b;
                println!("New Max: ({}, {}) -> {}", ma, mb, mx);
            }
        }
    }
    println!(
        "Best results were ({}, {}) -> {}. {} * {} = {}",
        ma,
        mb,
        mx,
        ma,
        mb,
        ma * mb
    );
}

fn count_primes(a: i32, b: i32) -> i32 {
    let mut n: i32 = 0;
    while (n * n + a * n + b) >= 0 && is_prime((n * n + a * n + b) as u64) {
        n += 1;
    }
    return n;
}
