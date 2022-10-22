use proconio::input;

fn main() {
    input! {
        mut n: f64,
        k: i32
    };

    for _ in 0..k {
        n = (n / 10f64).round();
    }

    println!("{}", n * 10f64.powi(k));
}
