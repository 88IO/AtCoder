use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, m: usize, a: [[usize]; m]
    };

    let mut ans = 0;

    for i in 1..2usize.pow(m as u32) {
        let mut s = HashSet::new();
        for j in 0..m {
            if (i >> j) & 1 == 1 {
                for k in a[j].iter() {
                    s.insert(k);
                }
            }
        }
        if s.len() == n {
            ans += 1;
        }
    }

    println!("{}", ans);
}
