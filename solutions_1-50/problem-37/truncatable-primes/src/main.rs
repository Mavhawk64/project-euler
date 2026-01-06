use primes::is_prime;
use std::collections::HashSet;
fn main() {
    let mut right: Vec<u64> = Vec::new();
    right.extend(generate_right_tree(2));
    right.extend(generate_right_tree(3));
    right.extend(generate_right_tree(5));
    right.extend(generate_right_tree(7));
    let set: HashSet<u64> = right.drain(..).collect(); // dedup
    right.extend(set);
    println!("{:?}", right);

    // Retry -- bool

    right.retain(|&x| is_left_truncatable(x));
    println!(
        "{:?}, {}, {}",
        right,
        right.len(),
        right.iter().sum::<u64>()
    );

    // Original Attempt: OVERFLOW
    // let mut left: Vec<u64> = Vec::new();
    // left.extend(generate_left_tree(2, 1));
    // left.extend(generate_left_tree(3, 1));
    // left.extend(generate_left_tree(5, 1));
    // left.extend(generate_left_tree(7, 1));

    // let set: HashSet<u64> = left.drain(..).collect(); // dedup
    // left.extend(set);
    // println!("{:?}", left);

    // let mut intersection: Vec<u64> = intersection(vec![left, right]);

    // println!("{:?}", intersection);
}

fn generate_right_tree(p: u64) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();
    if is_prime(10 * p + 1) {
        ret.push(10 * p + 1);
        ret.extend(generate_right_tree(10 * p + 1));
    }
    if is_prime(10 * p + 3) {
        ret.push(10 * p + 3);
        ret.extend(generate_right_tree(10 * p + 3));
    }
    if is_prime(10 * p + 7) {
        ret.push(10 * p + 7);
        ret.extend(generate_right_tree(10 * p + 7));
    }
    if is_prime(10 * p + 9) {
        ret.push(10 * p + 9);
        ret.extend(generate_right_tree(10 * p + 9));
    }
    return ret;
}

// RETRY -- TEST IF LEFT_TRUNCATABLE INSTEAD?
fn is_left_truncatable(p: u64) -> bool {
    let mut c: Vec<char> = p.to_string().chars().collect();
    while !c.is_empty() {
        if !is_prime(c.iter().collect::<String>().parse::<u64>().unwrap()) {
            return false;
        }

        c.remove(0);
    }
    return true;
}

// ORIGINAL ATTEMPT -- OVERFLOW

// fn generate_left_tree(p: u64, k: u32) -> Vec<u64> {
//     let mut ret: Vec<u64> = Vec::new();
//     let m: u64 = 10u64.pow(k); // cool new syntax I learned for pows of ten ret as u64
//     for d in 1..10 {
//         if is_prime(m * d + p) {
//             ret.push(m * d + p);
//             ret.extend(generate_left_tree(m * d + p, k + 1));
//         }
//     }
//     return ret;
// }

// Source - https://stackoverflow.com/a/72561054
// Posted by Jounathaen
// Retrieved 2026-01-05, License - CC BY-SA 4.0

// pub fn intersection(nums: Vec<Vec<u64>>) -> Vec<u64> {
//     let mut intersect_result: Vec<u64> = nums[0].clone();

//     for temp_vec in nums {
//         let unique_a: HashSet<u64> = temp_vec.into_iter().collect();
//         intersect_result = unique_a
//             .intersection(&intersect_result.into_iter().collect())
//             .map(|i| *i)
//             .collect::<Vec<_>>();
//     }
//     intersect_result
// }
