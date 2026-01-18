use std::collections::HashMap;

fn main() {
    let mut cubes: HashMap<[u8; 10], Vec<u64>> = HashMap::new();

    for n in 1..10_000 {
        let c = cube(n);
        let signature = num_to_digit_count(c);
        cubes.entry(signature).or_default().push(n);
        if cubes[&signature].len() == 5 {
            let first_root = cubes[&signature][0];
            println!("Found 5 permutations! Smallest cube: {}", cube(first_root));
            break;
        }
    }
}

fn cube(n: u64) -> u128 {
    (n as u128).pow(3)
}

fn num_to_digit_count(mut n: u128) -> [u8; 10] {
    let mut counts = [0u8; 10]; // malloc 10 slots of size u8, initialized to 0u8.
    while n > 0 {
        counts[(n % 10) as usize] += 1;
        n /= 10;
    }
    counts
}
