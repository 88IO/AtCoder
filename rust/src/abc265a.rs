use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize
    };

    if n < 3 || 3 * x < y {
        println!("{}", x * n);
    } else {
        println!("{}", x * (n % 3) + y * (n / 3));
    }
}
