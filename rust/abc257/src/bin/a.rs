use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    };

    println!("{}",  (65 + (x - 1) / n) as u8 as char);
}
