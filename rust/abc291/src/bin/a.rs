use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes
    };

    #[allow(non_snake_case)]
    let A = 65;
    #[allow(non_snake_case)]
    let Z = 90;

    for (i, c) in s.into_iter().enumerate() {
        if c >= A && c <= Z {
            println!("{}", i + 1);
        }
    }
}
