use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let s: Vec<String> = get_input();
    println!("{:#?}", s);

    // I'm pretty sure that I can get the first 10 digits of the sum by using
    // the first 13 digits (maybe even less!)
    // Let's think about it...
    // Anything that adds up to < 10 will not change any digits
    // However, if we have an absolute max of all 9's in a column (of digits)
    // Then, we would be at 9 * 100 = 0, 90 carry-over -> 0, 9 carry-over.
    // Therefore, we shift by 2 digits at the most, and we don't need to care about those.
    // I'll just do the first 13 digits to be safe and see if even smaller will also do the trick.

    // Cut down the Strings to grab the first 13 digits.
    let mut u: Vec<u128> = [].to_vec();
    for i in 0..s.len() {
        u.push(u128::from_str(&s[i][..12]).expect("Invalid string to be parsed to u128!"));
        // 11 seems to work (at least for this case)
        // According to https://projecteuler.net/thread=13 -- 11 digits is all that is required.

        // After some further thought, using 12 digits is the mathematical guarantee, and hence, what I will be sticking to in this file.
        // Let's consider the following:
        // Let N = # of (or, max # of, for varying sizes) digits of the numbers (i.e., 50 digits in this problem).
        // Let n = # of digits we want to view (from the front) of the summed output (i.e., 10 digits in this problem).
        // n <= N.
        // Let D = # of numbers to be summed (i.e., 100 numbers in this problem).
        // With some simple, but careful, calculations, we can find a nice upper bound for the number of required digits
        // to be significant for our solution.
        // Let's take a look at this current example.
        // We can find the maximum 50 digit number to be 10**50 - 1 (50 digits of 9).
        // If we multiply this by the number of numbers we have, we can find a maximum number summed:
        // 100 * (10**50 - 1) --> 52 digits long: 50 x 9, 2 x 0.
        // Let's analyze this a little closer:
        // Go back to the maximum "N"-digit number = 10**N - 1 (N digits of 9).
        // Multiply by D: D * (10**N - 1)
        // Let's consider taking only n digits. We lose information if there is ever a carry-over, so we basically need
        // n + max(# carry-overs) which can just be (loosely?) set to ceil(log(D))
        // Why is there a log/ceil? Well, the log essentially counts the number of 0's added to the end.
        // In our example, we had D = 100 numbers, so we would have log(100) = 2 digits appended to the end that would
        // (likely) get thrown away, as they do in the problem.
        // The ceil is just to be safe for cases where D != 10^x, for some x is a natural number [1,2,3,...]

        // Therefore, our final solution can be: L = n + ceil(log(D)), where L = # of digits to be tracked in the substring sum.
    }

    println!("{:#?}", u);
    let m: String = (&format!("{}", u.iter().sum::<u128>())[..10]).to_string();
    println!("{}", m);
}

fn get_input() -> Vec<String> {
    let mut v: Vec<String> = [].to_vec();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            v.push(line);
        }
    }
    return v;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn bad(n: Vec<u128>) -> u128 {
//     let mut s: u128 = 0;
//     for i in 0..n.len() {
//         s += n[i];
//     }
//     return s;
// }
