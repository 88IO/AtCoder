use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
    };

    let mut map = BTreeMap::new();

    for _ in 0..n {
        input! {
            a: usize
        };
        *map.entry(a).or_insert(0) += 1;
    }

    for c in map.values().rev() {
        println!("{}", c);
    }

    for _ in map.len()..n {
        println!("0");
    }
}
