use std::cmp::max;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;

fn main() {
    // let num1 = BigInteger::new("1234");
    // let num2 = BigInteger::new("9000");
    // let sum = &num1 + &num2;
    // println!("{} + {} = {}", num1, num2, sum);
    let num1 = BigInteger::new("1234");
    let nu32 = 56;
    let prod = num1.mult_u32(nu32);
    println!("{} * {} = {}", num1, nu32, prod);
    // println!("Sum of digits of {} = {}", num1, num1.sum_digits());
    let num = 100;
    println!("OLD : {}! = {}", num, factorial(num as u32));
    println!("TREE: {}! = {}", num, treefactorial(num as u32));

    println!(
        "Sum of the digits of {}! = {}",
        num,
        treefactorial(num as u32).sum_digits()
    );
}

// 100! -> 100 * 99 * ... * 3 * 2 * 1

// any number like:
// 1234 = 1 * thousand + 2 * hundred + 3 * ten + 4 * one
// multiplied by a small number (56)
// can be done like this:
// 56 * 1 * thousand + 56 * 2 * hundred + 56 * 3 * ten + 56 * 4 * one
// just have a list of digits to prevent overflow of u128
// however, we need to be smart about this to realize when we need to carry:
// 56 * 1 * thousand + 56 * 2 * hundred + 56 * 3 * ten + 56 * 4 * one
// = (5 * ten_thousand + 6 * thousand) + (10 * thousand + 12 * hundred) + (15 * hundred + 18 * ten) + (20 * ten + 24 * one)
// = 5 * ten_thousand + 16 * thousand + 27 * hundred + 38 * ten + 24 * one
// = 5 * ten_thousand + (1 * ten_thousand + 6 * thousand) + (2 * thousand + 7 * hundred) + (3 * hundred + 8 * ten) + (2 * ten + 4 * one)
// = 6 * ten_thousand + 8 * thousand + 10 * hundred + 10 * ten + 4 * one
// = 6 * ten_thousand + 9 * thousand + 0 * hundred + 1 * hundred + 0 * ten + 4 * one
// = 6 * ten_thousand + 9 * thousand + 1 * hundred + 0 * ten + 4 * one
// = 69104 ✓

// going back to these comments, I actually optimized my algorithm a little bit by doing the carry-over in the same step as the multiplication
// step, and I went from ones -> tens -> hundreds -> ..., so I wouldn't have to keep doing the same reduce step over and over again.

#[derive(Debug)]
struct BigInteger {
    digits: Vec<u8>,
}

impl BigInteger {
    fn new(value: &str) -> Self {
        let mut digits = Vec::new(); // create a new set of digits
        for c in value.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                digits.push(digit as u8);
            }
        }
        return BigInteger { digits };
    }

    fn from_u128(value: u128) -> Self {
        Self::new(&value.to_string())
    }

    fn add_(&self, other: &BigInteger) -> BigInteger {
        let mut result = Vec::new();
        let mut carry = 0;

        // Iterate through each element, adding one at a time:
        for i in 0..max(self.digits.len(), other.digits.len()) {
            let s = (if i < self.digits.len() {
                self.digits[i]
            } else {
                0
            }) + (if i < other.digits.len() {
                other.digits[i]
            } else {
                0
            }) + carry;
            carry = s / 10;
            let s = s % 10;
            result.push(s);
        }
        if carry > 0 {
            result.push(carry);
        }

        return BigInteger { digits: result };
    }

    // Cheers to Gemini for implementing this BigInteger * BigInteger method!
    // this section was not necessary for the problem, but it was interesting to explore.
    fn mul_(&self, other: &BigInteger) -> BigInteger {
        // Initialize result as zero
        let mut final_result = BigInteger::new("0");

        for (i, &digit2) in other.digits.iter().enumerate() {
            // 1. Multiply self by a single digit from 'other'
            let mut temp_prod = self.mult_u32(digit2 as u32);

            // 2. "Shift" it left by inserting i zeros at the front
            // (Because index 0 is the ones place, index 1 is tens, etc.)
            for _ in 0..i {
                temp_prod.digits.insert(0, 0);
            }

            // 3. Add this partial product to our final result
            final_result = final_result.add_(&temp_prod);
        }

        // Clean up any trailing zeros (e.g., if multiplying by 0)
        while final_result.digits.len() > 1 && final_result.digits.last() == Some(&0) {
            final_result.digits.pop();
        }

        return final_result;
    }

    fn mult_u32(&self, other: u32) -> BigInteger {
        // Multiply each digit with our small number
        let mut big_digits: Vec<u128> = self.digits.iter().map(|&d| d as u128).collect();
        // index=0 -> ones, index=1 -> tens, ..., index=n -> 10**n
        let mut carry: u128 = 0;
        for i in 0..big_digits.len() {
            let prod = big_digits[i] * (other as u128) + carry;
            big_digits[i] = prod % 10;
            carry = prod / 10;
        }
        // in the case where we have a huge carry (like 100), we need to carry over 0, 0, 1 (three times)
        while carry > 0 {
            big_digits.push(carry % 10);
            carry /= 10;
        }
        // now let's return a new BigInteger with our updated digits as u8 again.
        return BigInteger {
            digits: big_digits.iter().map(|&d| d as u8).collect(),
        };
    }

    fn sum_digits(&self) -> u128 {
        let mut sum: u128 = 0;
        for i in 0..self.digits.len() {
            sum += self.digits[i] as u128;
        }
        return sum;
    }
}

fn factorial(n: u32) -> BigInteger {
    if n == 0 {
        return BigInteger::new("1"); // only exception to the bottom.
    }
    let mut ret: BigInteger = BigInteger::from_u128(n as u128);
    for i in 1..n {
        ret = ret.mult_u32(i as u32);
    }
    return ret;
}

fn range_prod(lo: u32, hi: u32) -> BigInteger {
    if lo == hi {
        return BigInteger::from_u128(lo as u128);
    }
    if lo + 1 == hi {
        return BigInteger::from_u128((lo as u128) * (hi as u128));
    }

    let mid = lo + (hi - lo) / 2;
    &range_prod(lo, mid) * &range_prod(mid + 1, hi)
}

fn treefactorial(n: u32) -> BigInteger {
    if n < 2 {
        return BigInteger::new("1");
    }
    return range_prod(1, n);
}

// This translates to: "Add a reference to a BigInt to another reference"
// Thanks Gemini (Google AI)
impl<'a, 'b> Add<&'b BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn add(self, other: &'b BigInteger) -> BigInteger {
        self.add_(other)
    }
}

impl<'a, 'b> Mul<&'b BigInteger> for &'a BigInteger {
    type Output = BigInteger;
    fn mul(self, other: &'b BigInteger) -> BigInteger {
        self.mul_(other)
    }
}

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.digits.iter().rev().map(|d| d.to_string()).collect();
        write!(f, "{}", s)
    }
}
