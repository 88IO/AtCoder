use proconio::input;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    for _ in 0..5 {
        input! {
            n: usize
        };

        *map.entry(n).or_insert(0) += 1;
    }

    println!("{}", map.len());
}
