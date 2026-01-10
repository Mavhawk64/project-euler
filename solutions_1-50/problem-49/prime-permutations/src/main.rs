use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Sub;
fn main() {
    const L: usize = 10_000; // max 4-digit number
    let mut sieve: Vec<u16> = vec![0; L];
    for i in 2..L {
        if sieve[i] == 0 {
            for j in (2 * i..L).step_by(i) {
                sieve[j] += 1;
            }
        }
    }

    'outer: for i in 0..L {
        if sieve[i] == 0 {
            // prime
            let mut perms: Vec<u16> = get_permutations(i as u16);
            perms.retain(|&x| sieve[x as usize] == 0);
            // println!("{:?}", perms);
            for combo in perms.iter().combinations(3) {
                let is_ap: bool = is_arithmetic_progression(combo.iter().copied());
                // println!("{:?} {}", combo, is_ap);
                let mut sorted_combo = combo;
                sorted_combo.sort();
                if is_ap && *sorted_combo[0] != 1487 {
                    // println!("{:?} {}", sorted_combo, is_ap);
                    println!("{}{}{}", sorted_combo[0], sorted_combo[1], sorted_combo[2]);
                    break 'outer;
                }
            }
        }
    }
}

// https://youtu.be/UjEyLb_A3SY?si=_Vqz1r_Ebv_LDdOA&t=217
// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=9729edf14743236bf3e936225c8d7880

fn is_arithmetic_progression(
    nums: impl IntoIterator<Item: Copy + PartialOrd + Sub<Output: PartialEq>>,
) -> bool {
    nums.into_iter()
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .tuple_windows() //updated to tuple_windows() because map_windows() is nightly only
        .map(|(a, b)| b - a)
        .all_equal()
}

fn get_permutations(n: u16) -> Vec<u16> {
    let mut digits: Vec<u8> = number_to_vec(n as u32);
    let mut result = Vec::new();
    permute(&mut digits, 0, &mut result);
    result
        .into_iter()
        .collect::<HashSet<u16>>()
        .into_iter()
        .collect()
}

fn permute(digits: &mut Vec<u8>, start: usize, result: &mut Vec<u16>) {
    if start == digits.len() {
        let num = digits
            .iter()
            .fold(0u16, |acc, &digit| acc * 10 + digit as u16);
        result.push(num);
        return;
    }

    for i in start..digits.len() {
        if start == 0 && digits[i] == 0 {
            continue;
        }

        digits.swap(start, i);
        permute(digits, start + 1, result);
        digits.swap(start, i);
    }
}

// https://users.rust-lang.org/t/how-to-convert-a-number-to-numeric-vec/10404/2
fn number_to_vec(n: u32) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
}
