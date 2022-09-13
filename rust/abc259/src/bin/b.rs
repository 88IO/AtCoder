use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64
    };

    let r = d / 180f64 * PI;

    println!("{} {}", a * r.cos() - b * r.sin(), a * r.sin() + b * r.cos());
}
