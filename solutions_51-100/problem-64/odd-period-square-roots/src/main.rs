fn main() {
    let mut c: usize = 0;
    for N in 0..=10_000 {
        c += if is_odd_rep(N) { 1 } else { 0 };
    }
    println!("{}", c);
}

fn m_next(a: i32, d: i32, m: i32) -> i32 {
    d * a - m
}

fn d_next(N: i32, m_next: i32, d: i32) -> i32 {
    (N - m_next * m_next) / d
}

fn a_next(a_0: i32, m_next: i32, d_next: i32) -> i32 {
    (a_0 + m_next) / d_next
}

fn is_odd_rep(N: i32) -> bool {
    let a_0: i32 = N.isqrt();
    if a_0 * a_0 == N {
        return false;
    }
    let mut a_n: i32 = a_0;
    let mut m_n = 0;
    let mut d_n = 1;
    let mut cnt: usize = 1;
    loop {
        let m_n1 = m_next(a_n, d_n, m_n);
        let d_n1 = d_next(N, m_n1, d_n);
        let a_n1 = a_next(a_0, m_n1, d_n1);
        if a_n1 == 2 * a_0 {
            break;
        }
        cnt += 1;
        a_n = a_n1;
        d_n = d_n1;
        m_n = m_n1;
    }

    !cnt.is_multiple_of(2)
}
