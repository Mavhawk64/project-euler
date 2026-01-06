fn main() {
    // since we're finding the sum of all nums (in base 10) of double-base palindromes (b_10, b_2), we can skip 0
    // start with 1 and go every odd number (+=2)
    let mut s: u32 = 0;
    for j in 0..500_000 {
        let i: u32 = 2 * j + 1;
        let b: String = format!("{:b}", i);
        if is_palindrome(i.to_string()) && is_palindrome(b) {
            s += i;
        }
    }
    println!("{}", s);
}

fn is_palindrome(x: String) -> bool {
    let mut c: Vec<char> = x.chars().collect();
    while !c.is_empty() {
        if c.len() == 1 {
            break;
        }
        let f: char = c.remove(0);
        let e: Option<char> = c.pop();
        if Some(f) != e {
            return false;
        }
    }
    return true;
}
