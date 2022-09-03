use std::collections::HashSet;
use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        mut a: [usize; n]
    };

    let mut set = HashSet::new();
    a.push(0); a.push(0);

    for comb in a.iter().combinations(3) {
        let n: usize = comb.into_iter().sum();
        if n <= w {
            set.insert(n);
        }
    }

    println!("{}", set.len());
}
