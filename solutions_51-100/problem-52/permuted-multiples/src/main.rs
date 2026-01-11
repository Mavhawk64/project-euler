fn main() {
    let mut x: u64 = 1;
    loop {
        let mut o: Vec<u8> = number_to_vec(x);
        let mut t: Vec<u8> = number_to_vec(2 * x);
        let mut h: Vec<u8> = number_to_vec(3 * x);
        let mut f: Vec<u8> = number_to_vec(4 * x);
        let mut i: Vec<u8> = number_to_vec(5 * x);
        let mut s: Vec<u8> = number_to_vec(6 * x);
        o.sort();
        t.sort();
        h.sort();
        f.sort();
        i.sort();
        s.sort();
        let n: u64 = vec_to_number(o);
        let w: u64 = vec_to_number(t);
        let r: u64 = vec_to_number(h);
        let u: u64 = vec_to_number(f);
        let v: u64 = vec_to_number(i);
        let c: u64 = vec_to_number(s);
        if n == w && w == r && r == u && u == v && v == c {
            println!("{}", x);
            return;
        }
        x += 1;
    }
}

fn number_to_vec(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
}

fn vec_to_number(v: Vec<u8>) -> u64 {
    v.into_iter()
        .fold(0u64, |acc: u64, d: u8| 10 * acc + d as u64)
}
