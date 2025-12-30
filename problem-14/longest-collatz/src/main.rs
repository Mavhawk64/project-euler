fn main() {
    let mut b: u128 = 1;
    let mut bc: u128 = 0;
    for i in 1..(1e6 as u128) {
        let c: u128 = get_collatz_count(i);
        if c > bc {
            b = i;
            bc = c;
            println!("{} gives us a new highest of {}!", i, c);
        }
    }
    println!("{}", b);
}

fn get_collatz_count(mut n: u128) -> u128 {
    let mut c: u128 = 0;
    while n != 1 {
        n = collatz(n);
        c += 1;
    }
    return c;
}

fn collatz(n: u128) -> u128 {
    return if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
}
