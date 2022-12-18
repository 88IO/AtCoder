use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize, w: usize
    };

    let mut vs = vec![vec!['0'; h]; w];
    let mut vt = vec![vec!['0'; h]; w];

    for j in 0..h {
        input! {
            s: Chars
        };
        for (i, si) in s.into_iter().enumerate() {
            vs[i][j] = si;
        }
    }
    for j in 0..h {
        input! {
            t: Chars
        };
        for (i, ti) in t.into_iter().enumerate() {
            vt[i][j] = ti;
        }
    }

    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();

    for i in 0..w {
        *map_s.entry(&vs[i]).or_insert(0) += 1;
        *map_t.entry(&vt[i]).or_insert(0) += 1;
    }

    if map_s == map_t {
        println!("Yes");
    } else {
        println!("No");
    }
}
