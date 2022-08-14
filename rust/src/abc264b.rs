use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        r: isize,
        c: isize
    };

    let x = (8 - r).abs();
    let y = (8 - c).abs();

    if max(x, y) % 2 == 0 {
        println!("white");
    } else {
        println!("black");
    }
}
