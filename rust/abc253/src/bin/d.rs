use proconio::input;
use num::integer;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    };

    let c = integer::lcm(a, b);


    let n_a = n / a;
    let n_b = n / b;
    let n_c = n / c;

    let s_a = a * n_a;
    let s_b = b * n_b;
    let s_c = c * n_c;

    let mut s = n * (n + 1) / 2;
    s += n_c * (c + s_c) / 2;
    s -= n_a * (a + s_a) / 2;
    s -= n_b * (b + s_b) / 2;

    println!("{}", s);
}
