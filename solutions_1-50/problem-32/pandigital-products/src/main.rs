fn main() {
    let mut li: Vec<u32> = Vec::new();
    // Range 1000..10000 is correct because n must be 4 digits
    // to allow for 5 digits between the two factors (total 9).
    for i in 1000..10000 {
        if are_digits_unique(i) {
            li.push(i);
        }
    }

    li.retain(|&x| !contains_zero(x));
    li.retain(|&x| has_unique_pandigital_factors_that_multiply_to(x));

    println!("len = {}, sum = {}", li.len(), li.iter().sum::<u32>());
}

fn are_digits_unique(mut n: u32) -> bool {
    if n == 0 {
        return true;
    }
    if n > 987654321 {
        return false;
    }

    let mut mask = 0u16;
    while n > 0 {
        let digit = n % 10;
        let bit = 1 << digit;
        if (mask & bit) > 0 {
            return false;
        }
        mask |= bit;
        n /= 10;
    }
    return true;
}

fn contains_zero(mut n: u32) -> bool {
    if n == 0 {
        return true;
    }
    while n > 0 {
        if n % 10 == 0 {
            return true;
        }
        n /= 10;
    }
    return false;
}

fn get_factors(n: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    for i in 1..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            v.push(i);
            if i * i != n {
                v.push(n / i);
            }
        }
    }
    v.sort();
    return v;
}

fn contains_shared_digits(mut a: u32, mut b: u32) -> bool {
    let mut mask: u16 = 0;
    while a > 0 {
        mask |= 1 << (a % 10);
        a /= 10;
    }
    while b > 0 {
        if (mask & (1 << (b % 10))) != 0 {
            return true;
        }
        b /= 10;
    }
    return false;
}

fn count_digits(mut n: u32) -> u32 {
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    return count;
}

fn has_unique_pandigital_factors_that_multiply_to(n: u32) -> bool {
    let n_digits = count_digits(n);
    let mut factors = get_factors(n);

    factors.retain(|&x| are_digits_unique(x));
    factors.retain(|&x| !contains_zero(x));
    factors.retain(|&x| !contains_shared_digits(x, n));

    if factors.is_empty() {
        return false;
    }

    for &x in &factors {
        let frac = n / x;

        // NEW LOGIC: We must check if the sum of digits of x, frac, and n is exactly 9
        let total_digits = count_digits(x) + count_digits(frac) + n_digits;

        if total_digits == 9 && factors.contains(&frac) && !contains_shared_digits(x, frac) {
            return true;
        }
    }
    return false;
}
