fn main() {
    let mut total_sum_set = std::collections::HashSet::new();

    // The product 'n' can't realistically exceed 4 digits if the
    // total digits (x * y = n) must equal 9.
    for n in 1..10000 {
        if has_unique_pandigital_factors(n) {
            total_sum_set.insert(n);
        }
    }

    println!("Sum: {}", total_sum_set.iter().sum::<u32>());
}

fn has_unique_pandigital_factors(n: u32) -> bool {
    let n_mask = get_digit_mask(n);
    // If n has 0 or duplicate digits, it's invalid
    if n_mask == 0 {
        return false;
    }

    let limit = (n as f32).sqrt() as u32;
    for x in 1..=limit {
        if n % x == 0 {
            let y = n / x;
            if is_pandigital_9(x, y, n) {
                return true;
            }
        }
    }
    false
}

// Returns 0 if digits are not unique or contain 0, else returns bitmask
fn get_digit_mask(mut n: u32) -> u16 {
    let mut mask = 0u16;
    while n > 0 {
        let digit = n % 10;
        if digit == 0 {
            return 0;
        } // Problem 32 is 1-9 only
        let bit = 1 << digit;
        if (mask & bit) != 0 {
            return 0;
        } // Duplicate found
        mask |= bit;
        n /= 10;
    }
    mask
}

fn is_pandigital_9(x: u32, y: u32, n: u32) -> bool {
    let m1 = get_digit_mask(x);
    if m1 == 0 {
        return false;
    }

    let m2 = get_digit_mask(y);
    if m2 == 0 {
        return false;
    }

    let m3 = get_digit_mask(n);
    if m3 == 0 {
        return false;
    }

    // Check if all three masks share NO bits
    if (m1 & m2) != 0 || (m1 & m3) != 0 || (m2 & m3) != 0 {
        return false;
    }

    // Combined mask must be exactly digits 1-9 (binary 1111111110)
    // 1<<1 | 1<<2 | ... | 1<<9 = 1022
    (m1 | m2 | m3) == 1022
}
