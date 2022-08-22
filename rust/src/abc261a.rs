use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        l1: isize,
        r1: isize,
        l2: isize,
        r2: isize
    };

    let a = max(l1, l2);
    let b = min(r1, r2);

    println!("{}", max(b - a, 0));
}
