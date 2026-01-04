use factorial::Factorial; // Ensure you ran `cargo add factorial`

fn main() {
    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{}", find_permutations(digits, 1_000_000));
}

fn find_permutations(digits: Vec<u8>, n: usize) -> String {
    let mut l: u128 = (n - 1) as u128;
    let mut result: Vec<u8> = Vec::new();
    let mut available_digits: Vec<u8> = digits.clone();
    let mut i = (digits.len() as isize) - 1;
    while i >= 0 {
        let fact = (i as usize).factorial() as u128;
        let index = (l / fact) as usize;
        result.push(available_digits.remove(index));
        l %= fact;
        i -= 1;
    }

    return result.iter().map(|&x| x.to_string()).collect();
}
