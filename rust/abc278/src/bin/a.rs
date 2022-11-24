use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    };

    for i in k..n {
        print!("{} ", a[i]);
    }
    for _ in 0..k.min(n) {
        print!("0 ");
    }
    println!();
}
