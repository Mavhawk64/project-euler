fn main() {
    let mut an = AmicableNumbers::new();
    for i in 1..10000 {
        an.push_u128(i as u128);
    }
    let ap = an.get_pairs();
    println!("Total sum of amicable numbers: {}", ap.sum());
}

fn d(n: u128) -> u128 {
    return proper_divisors(n).iter().sum();
}

fn proper_divisors(n: u128) -> Vec<u128> {
    if n < 2 {
        return vec![];
    }
    let mut v: Vec<u128> = Vec::new();
    let limit = n.isqrt();
    for i in 1..=limit {
        if n % i == 0 {
            v.push(i);
            let counterpart = n / i;
            if counterpart != n && counterpart != i {
                v.push(counterpart);
            }
        }
    }
    return v;
}

#[derive(Debug, Clone, Copy)]
struct AmicableNumber {
    n: u128,
    sum_pd: u128,
}

impl AmicableNumber {
    fn new(value: u128) -> Self {
        AmicableNumber {
            n: value,
            sum_pd: d(value),
        }
    }
}

#[derive(Debug)]
struct AmicableNumbers {
    nums: Vec<AmicableNumber>,
}

impl AmicableNumbers {
    fn new() -> Self {
        return AmicableNumbers { nums: Vec::new() };
    }

    fn push_u128(&mut self, n: u128) {
        self.nums.push(AmicableNumber::new(n));
    }

    // Rust uses Option<usize> instead of -1
    // Thanks Gemini for the refactoring!
    fn index_of(&self, target_n: u128) -> Option<usize> {
        return self.nums.iter().position(|x| x.n == target_n);
    }

    fn get_pairs(&self) -> AmicablePairs {
        let mut p = AmicablePairs::new();
        for i in 0..self.nums.len() {
            let current = self.nums[i];
            if current.sum_pd > current.n {
                if let Some(idx) = self.index_of(current.sum_pd) {
                    let potential_partner = self.nums[idx];
                    if potential_partner.sum_pd == current.n {
                        p.push(AmicablePair {
                            a: current.n,
                            b: potential_partner.n,
                        });
                    }
                }
            }
        }
        return p;
    }
}

#[derive(Debug, Clone, Copy)]
struct AmicablePair {
    a: u128,
    b: u128,
}

impl AmicablePair {
    fn sum(&self) -> u128 {
        self.a + self.b
    }
}

#[derive(Debug)]
struct AmicablePairs {
    pairs: Vec<AmicablePair>,
}

impl AmicablePairs {
    fn new() -> Self {
        return AmicablePairs { pairs: Vec::new() };
    }

    fn push(&mut self, a: AmicablePair) {
        self.pairs.push(a);
    }

    fn sum(&self) -> u128 {
        let mut s: u128 = 0;
        for i in 0..self.pairs.len() {
            s += self.pairs[i].sum();
        }
        return s;
    }
}
