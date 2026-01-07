use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/home/maverick/repos/project-euler/solutions_1-50/problem-42/coded-triangle-numbers/src/words.txt";
    let content = fs::read_to_string(path)?;
    let words: Vec<String> = content
        .split(',')
        .map(|s: &str| s.replace('"', ""))
        .collect();

    let mut cnt: usize = 0;
    for i in words {
        let t: u64 = i.chars().map(|c: char| c as u64 - 64).sum();
        if t == get_nth_triangle_number(get_n(t)) {
            println!("{}", i);
            cnt += 1;
        }
    }
    println!("{}", cnt);

    Ok(())
}

fn get_nth_triangle_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn get_n(t: u64) -> u64 {
    ((((8 * t + 1) as f64).sqrt() - 1.0) / 2.0) as u64
} // (PURPOSELY) INVALIDLY PLACES NON-TRIANGULAR NUMBERS -- EASY CHECK!
