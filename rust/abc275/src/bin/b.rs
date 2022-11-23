use proconio::input;

fn main() {
    input! {
        mut a: usize, mut b: usize, mut c: usize,
        mut d: usize, mut e: usize, mut f: usize
    };

    println!("{}", (998244353 + mul_q(mul_q(a, b), c) % 998244353 - mul_q(mul_q(d, e), f) % 998244353) % 998244353);
}

fn mul_q(x: usize, y: usize) -> usize {
    (x % 998244353) * (y % 998244353)
}
