use itertools::Itertools;

fn main() {
    let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let pandigitals = generate_pandigitals(input, 10);
    let mut s: u128 = 0;
    for pd in pandigitals {
        // I don't want to do this, but let's convert to a string, since it's literally in the name of the puzzle!
        let d: Vec<u64> = pd
            .to_string()
            .chars()
            .map(|c: char| c.to_digit(10).unwrap() as u64)
            .collect();
        if (100 * d[1] + 10 * d[2] + d[3]).is_multiple_of(2)
            && (100 * d[2] + 10 * d[3] + d[4]).is_multiple_of(3)
            && (100 * d[3] + 10 * d[4] + d[5]).is_multiple_of(5)
            && (100 * d[4] + 10 * d[5] + d[6]).is_multiple_of(7)
            && (100 * d[5] + 10 * d[6] + d[7]).is_multiple_of(11)
            && (100 * d[6] + 10 * d[7] + d[8]).is_multiple_of(13)
            && (100 * d[7] + 10 * d[8] + d[9]).is_multiple_of(17)
        {
            println!("{}", pd);
            s += pd as u128;
        }
    }
    println!("Sum:\n{}", s);
}

// Due to the continued use of pandigitals of differing variety, I found that rust has a package
// similar to python's itertools, that goes by the same name.
// This is a nice implementation of iterating across each of the digits to find the pandigitals.
fn generate_pandigitals(digits: Vec<u32>, fixed_length: usize) -> Vec<u64> {
    let mut results = Vec::new();
    let n = digits.len();
    for length in 2..=n {
        for p in digits.iter().permutations(length) {
            if *p[0] == 0 {
                continue;
            }
            let num = p.iter().fold(0u64, |acc, &&d| acc * 10 + d as u64);
            if num_digits(num) as usize == fixed_length {
                results.push(num);
            }
        }
    }

    results.sort(); // Optional: keep them in order
    results
}

fn num_digits(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (n.ilog10() + 1) as u64
    }
}
