fn main() {
    // What is the sum of the digits of 2^1000 ?
    // println!("{}", u128::MAX); // much smaller than 2**1000...

    // I think we need to either do some base magic or binary magic...
    // (one day later) maybe not...
    // I did a LeetCode problem with LinkNodes (like a number, 1234 being represented as 4->3->2->1)
    // and I was thinking I would create my own version of this for this problem and just do the digit by digit doubling
    // but then I thought, "I can do this with an array".
    // So here is a solution with an array that doubles the digits and progressively adds upon the previous list of digits.
    println!("{}", sum_2_pow(1000));
}

fn sum_2_pow(n: u32) -> u32 {
    let mut digits: Vec<u8> = vec![1];

    for _ in 0..n {
        let mut carry: u8 = 0;

        for d in digits.iter_mut() {
            let t: u16 = 2 * (*d as u16) + (carry as u16);
            *d = (t % 10) as u8;
            carry = (t / 10) as u8;
        }

        if carry != 0 {
            digits.push(carry);
        }
    }

//     let mut o: u128=0;
//     for &d in digits.iter() {
//     o += d as u128;
// }
// same result as below w/ mapping each val to u128:

    return digits.iter().map(|&d| d as u32).sum(); // a little sum magic w/ map courtesy of ChatGPT
}


// Python 3.14.1 (main, Dec  2 2025, 12:51:37) [GCC 12.3.0] on linux
// Type "help", "copyright", "credits" or "license" for more information.
// Ctrl click to launch VS Code Native REPL
// >>> 2**1000
// 10715086071862673209484250490600018105614048117055336074437503883703510511249361224931983788156958581275946729175531468251871452856923140435984577574698574803934567774824230985421074605062371141877954182153046474983581941267398767559165543946077062914571196477686542167660429831652624386837205668069376
// >>> sum([int(i) for i in list(str(2**1000))])
// 1366
// >>>
