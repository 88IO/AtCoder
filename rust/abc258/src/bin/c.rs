use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars
    };

    let mut t = 0;

    for _ in 0..q {
        input! {
            k: usize,
            x: usize
        };

        match k {
            1 => {
                t += x;
            },
            2 => {
                t %= n;
                println!("{}", s[(n - t + x - 1) % n]);
            },
            _ => {
                panic!("unexpected query");
            }
        }
    }
}
