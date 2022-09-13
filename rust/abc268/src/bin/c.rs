use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: isize,
    };

    let mut map = HashMap::new();

    for i in 0..n {
        input! {
            p: isize
        };

        *map.entry((p - 1 - i + n) % n).or_insert(0) += 1;
        *map.entry((p - i + n) % n).or_insert(0) += 1;
        *map.entry((p + 1 - i + n) % n).or_insert(0) += 1;
    }

    println!("{}", map.values().max().unwrap());
}
