use std::collections::HashSet;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    };

    let mut set: HashSet<(usize, usize)> = HashSet::new();

    let mut i = 1;
    let mut j = 1;

    set.insert((i, j));

    loop {
        match g[i - 1][j - 1] {
            'U' => {
                if i == 1 { break; }
                i -= 1;
            },
            'D' => {
                if i == h { break; }
                i += 1;
            },
            'L' => {
                if j == 1 { break; }
                j -= 1;
            },
            'R' => {
                if j == w { break; }
                j += 1;
            },
            _ => {
                panic!("unexpected character.");
            }
        }

        if set.contains(&(i, j)) {
            println!("-1");
            return;
        }

        set.insert((i, j));
    }

    println!("{} {}", i , j);
}
