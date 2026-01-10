use primes::is_prime;
use std::collections::HashSet;
fn main() {
    const RUN_OLD: bool = false;
    new_sol();

    if RUN_OLD {
        old_sol();
    }
}

fn new_sol() {
    // This method is a little bit more number-theory savvy
    // After learning about a Modified Sieve method
    // and Prime Number Theorem, we can see that the density of primes
    // around a number x is \rho(x)\approx\ln^{-1}x.
    // But we're not looking for primes directly, we're looking for \omega(n)=4.
    // This can be helped with the Erdos-Kac theorem:
    // https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93Kac_theorem
    // If we set our upper limit of 1e6, which is probably (definitely)
    // dense enough to search for \omega(n) = 4.
    // c.f. https://oeis.org/A001221
    // const L: usize = 150_000; // set to 150,000 to further optimize the code by >6x
    const L: usize = 150_000;
    let mut sieve: Vec<u32> = vec![0; L];
    for i in 2..L {
        if sieve[i] == 0 {
            // hit a prime
            // println!("{} is prime!", i);
            for j in (2 * i..L).step_by(i) {
                sieve[j] += 1;
            }
        }
    }

    // Loop through to find where i,i+1,i+2,i+3 are all 4.
    for i in 0..L {
        if sieve[i] == 4
            && sieve[i] == sieve[i + 1]
            && sieve[i + 1] == sieve[i + 2]
            && sieve[i + 2] == sieve[i + 3]
        {
            println!("{}", i);
            break;
        }
    }
}

fn old_sol() {
    let mut i: u64 = 2 * 3 * 5 * 7;
    loop {
        println!("{}", i);
        let frst: usize = prime_factorization(i)
            .into_iter()
            .collect::<HashSet<u64>>()
            .len();
        let scnd: usize = prime_factorization(i + 1)
            .into_iter()
            .collect::<HashSet<u64>>()
            .len();
        let thrd: usize = prime_factorization(i + 2)
            .into_iter()
            .collect::<HashSet<u64>>()
            .len();
        let frth: usize = prime_factorization(i + 3)
            .into_iter()
            .collect::<HashSet<u64>>()
            .len();
        // check if fourth has 4 uniques.
        if frth != 4 {
            i += 4;
            continue;
        }
        // check if third and fourth DON'T match up
        if thrd != frth {
            i += 3;
            continue;
        }
        // check if second and third DON'T match up
        if scnd != thrd {
            i += 2;
            continue;
        }
        // check if first and second DON'T match up
        if frst != scnd {
            i += 1;
            continue;
        }

        // Should be the end -->
        println!("{}: {} {} {} {}", i, frst, scnd, thrd, frth);
        break;
    }
}

// Since there is a prime_factorization crate, I'm going to try 2 methods: one by myself, and one with the crate.
// actually, i didn't
fn prime_factorization(n: u64) -> Vec<u64> {
    let mut p: u64 = 2;
    let mut m: u64 = n;
    let mut v: Vec<u64> = Vec::new();
    while m > 1 {
        while m.is_multiple_of(p) {
            m /= p;
            v.push(p);
        }
        p = next_prime(p);
    }
    v
}

fn next_prime(mut p: u64) -> u64 {
    p += 1;
    while !is_prime(p) {
        p += 1;
    }
    p
}
