fn main() {
    let mut lychrel_nums: Vec<u128> = Vec::new();
    for i in 1..10_001 {
        let mut is_lychrel: bool = true;
        let mut num: u128 = i;
        for _ in 0..50 {
            let reversed_num: u128 = num
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse()
                .unwrap();
            num = reversed_num + num;
            if is_palindrome(num.to_string()) {
                is_lychrel = false;
                break;
            }
        }
        if is_lychrel {
            lychrel_nums.push(i);
        }
    }
    println!("{:?}\n{}", lychrel_nums, lychrel_nums.len());
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
    true
}
