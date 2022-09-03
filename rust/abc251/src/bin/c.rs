use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut set = HashSet::new();
    let mut originals = Vec::new();

    for i in 1..=n {
        input! {
            s: String,
            t: usize
        };

        if !set.contains(&s) {
            set.insert(s);
            originals.push((i, t));
        }
    }

    let best = originals.iter()
        .max_by_key(|v| (v.1 + 1) * 1_000_000 - v.0)
        .unwrap();

    println!("{}", best.0);
}
