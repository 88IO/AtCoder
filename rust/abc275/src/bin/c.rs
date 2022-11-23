use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: [Chars; 9]
    };

    let mut counter = 0;

    for i in 0..8 {
        for j in 0..8 {
            for k in 1..(9 - i.max(j)) {
                for o in 0..k {
                    if c[i][j + o] == '#' && c[i + o][j + k] == '#'
                        && c[i + k - o][j] == '#' && c[i + k][j + k - o] == '#' {
                        counter += 1;
                    }
                }
            }
        }
    }

    println!("{}", counter);
}
