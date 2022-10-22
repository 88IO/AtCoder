use proconio::input;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize
    };

    let mut set: HashSet<Vec<usize>> = (1..=n).combinations(2).collect();


    for _ in 0..m {
        input! {
            k: usize,
            x: [usize; k]
        };

        for comb in x.into_iter().combinations(2) {
            set.remove(&comb);
        }
    }

    println!("{}", if set.is_empty() { "Yes" } else { "No" });
}
