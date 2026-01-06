use std::collections::HashSet;

fn main() {
    let mut m: usize = 0;
    let mut idx: u32 = 0;
    for i in 1..1001 {
        let out = get_right_triangles(i).len();
        if out > m {
            m = out;
            idx = i;
        }
    }

    println!("p = {}:\n{:#?}", idx, get_right_triangles(idx));
}

fn get_right_triangles(p: u32) -> HashSet<Vec<u32>> {
    let mut ret: HashSet<Vec<u32>> = HashSet::new();

    for a in 1..p / 2 {
        if ((p - 2 * a) * p).is_multiple_of(2 * (p - a)) {
            let b = ((p - 2 * a) * p) / (2 * (p - a)); // if this is not an int, then it's invalid!
            let mut tmp: Vec<u32> = vec![a, b, p - a - b];
            tmp.sort();
            ret.insert(tmp);
        }
    }
    ret
}

// a*a + b*b = c*c
// a + b + c = p
// c = p - a - b
// plug in and solve for b, leaving p (input) and a (loop) free
// b = (2a-p) * p / (2*a-p) --> PROBLEM: This errors out for u32
// b = (p - 2a)p / (p-a)
// Req: p >= 2a -> a <= p/2
