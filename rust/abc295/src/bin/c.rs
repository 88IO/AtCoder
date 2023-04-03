use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize
    };

    let mut map = HashMap::new();
    let mut count = 0;

    for _ in 0..n {
        input! {
            a: usize
        };

        *map.entry(a).or_insert(0) += 1;
    }

    for v in map.values() {
        count += v / 2;
    }

    println!("{}", count);
}
