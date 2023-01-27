use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize, a: usize, b: usize,
        s: Bytes
    };

    let mut yen_min = std::usize::MAX;

    for i in 0..n {
        let mut yen = a * i;

        for j in 0..(n / 2) {
            if s[(j + i) % n] != s[(n - 1 - j + i) % n] {
                yen += b;
            }
        }

        if yen < yen_min {
            yen_min = yen;
        }
    }

    println!("{}", yen_min);
}
