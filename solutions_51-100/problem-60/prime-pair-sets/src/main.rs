use primes::is_prime;
use std::collections::{HashMap, HashSet};

const LIMIT: u64 = 10_000;
fn main() {
    // A very naive approach would be to try and do a quintuple for-loop, brute forcing to see if this.
    // What we should do is an adjacency list (graph theory)
    // this would look like {3: [7,109,673], ...,7: [3,109,673],...}
    // We'd want to then go prime by prime until we find that sweet 5-prime set!
    let mut adj: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut primes: Vec<u64> = Vec::new();
    for i in 0..LIMIT {
        if is_prime(i) {
            primes.push(i);
            adj.insert(i, Vec::new());
        }
    }

    for i in 0..primes.len() {
        let p: u64 = primes[i];
        for &p2 in &primes[i + 1..] {
            if !(p + p2).is_multiple_of(3) && is_prime(concat(p, p2)) && is_prime(concat(p2, p)) {
                adj.get_mut(&p).unwrap().push(p2);
                adj.get_mut(&p2).unwrap().push(p);
            }
        }
    }

    // find the 5 primes via intersection!
    for &p1 in &primes {
        if let Some(adj_p1) = adj.get(&p1) {
            for &p2 in adj_p1 {
                let a1: HashSet<u64> = HashSet::from_iter(adj_p1.iter().cloned());
                let a2: HashSet<u64> = HashSet::from_iter(adj.get(&p2).unwrap().iter().cloned());
                let int_a12: HashSet<u64> = a1.intersection(&a2).cloned().collect();

                for p3 in int_a12.iter() {
                    // Fixed .to_iter() -> .iter()
                    let a3: HashSet<u64> = HashSet::from_iter(adj.get(p3).unwrap().iter().cloned());
                    let int_a123: HashSet<u64> = int_a12.intersection(&a3).cloned().collect();

                    for p4 in int_a123.iter() {
                        // Fixed .to_iter() -> .iter()
                        let a4: HashSet<u64> =
                            HashSet::from_iter(adj.get(p4).unwrap().iter().cloned());
                        let int_a1234: HashSet<u64> = int_a123.intersection(&a4).cloned().collect();

                        for p5 in int_a1234.iter() {
                            // Fixed .to_iter() -> .iter()
                            println!(
                                "{}, {}, {}, {}, {}\nSum:\n{}",
                                p1,
                                p2,
                                p3,
                                p4,
                                p5,
                                p1 + p2 + p3 + p4 + p5 // Problem asks for sum of the primes
                            );
                        }
                    }
                }
            }
        }
    }
}

fn concat(num1: u64, num2: u64) -> u64 {
    num1 * 10u64.pow(num_len(num2)) + num2
}

fn num_len(num: u64) -> u32 {
    if num == 0 {
        1
    } else {
        1 + num.ilog10()
    }
}
