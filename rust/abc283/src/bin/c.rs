use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes
    }

    let mut flag0 = false;
    let mut counter = 0;

    for b in s.into_iter() {
        if b == 48 {
            if !flag0 { counter += 1; }
            flag0 = !flag0;
        } else {
            counter += 1;
            flag0 = false;
        }
    }

    println!("{}", counter);
}
