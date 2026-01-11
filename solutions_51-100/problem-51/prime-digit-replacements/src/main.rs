use std::collections::HashSet;

use itertools::Itertools;
use primes::is_prime;

const ALLOW_LEADING_ZEROS: bool = false;

// Technically, I don't think this is a mathematically sound solution...
// maybe it is... idk
// Pitfalls that I can think of:
// Program stops at 120383 because it finds this pattern:
// *2*3*3 therefore return -> 121313
// However, can there be any numbers between 120383 and 121313 that could win?
fn main() {
    let mut i: u64 = 1;
    loop {
        if !is_prime(i) {
            i += 1;
            continue;
        }
        let count: usize = num_len(i);
        let mut indices_vecs: Vec<Vec<usize>> = (0..count).powerset().collect();
        indices_vecs.remove(0);
        println!("-- {} -- ", i);
        for indices in &indices_vecs {
            let mut result: Vec<u64> = wild_card(i, indices.clone()).into_iter().collect();
            result.retain(|&d| is_prime(d));
            // result.sort();
            println!("  {}", result.len());
            if result.len() >= 8 {
                // can't include itself ;-;
                result.sort();
                println!("{}", result[0]);
                println!("Proof: {:?}", result);
                println!("Wild Card indices: {:?}", indices);
                return;
            }
        }
        i += 1;
    }
}

fn num_len(n: u64) -> usize {
    if n == 0 {
        1
    } else {
        (n.ilog10() + 1) as usize
    }
}

// We need to get a prime number, say:
// 6101
// we can replace 1-len()-1 digits with the same digit.
// for instance:
// *101 -> 0101?, 1101, 2101, 3101, ..., 9101
// 6*01 -> 6001, 6101, 6201, ..., 6901
// ...
// **01 -> 0001?, 1101, 2201, 3301, ..., 9901
// *1*1 -> 0101?, 1111, 2121, 3131, ..., 9191
// ...
// ***1 -> 0001?, 1111, 2221, 3331, ..., 9991
fn wild_card(n: u64, indices: Vec<usize>) -> HashSet<u64> {
    let mut ret: HashSet<u64> = HashSet::new();
    let digits: Vec<u8> = number_to_vec(n);
    // ret.insert(n);
    for j in 0..10 {
        let mut tmp: Vec<u8> = digits.to_vec();
        for i in &indices {
            tmp[*i] = j as u8;
        }
        if number_to_vec(vec_to_number(tmp.clone())).len() == digits.len() || ALLOW_LEADING_ZEROS {
            ret.insert(vec_to_number(tmp));
        }
    }
    ret
}

fn number_to_vec(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
}

fn vec_to_number(v: Vec<u8>) -> u64 {
    v.into_iter()
        .fold(0u64, |acc: u64, d: u8| 10 * acc + d as u64)
}
