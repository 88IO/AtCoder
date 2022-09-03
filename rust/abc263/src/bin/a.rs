use std::collections::HashMap;

use proconio::input;

fn main() {
    let mut map = HashMap::new();

    for _ in 0..5 {
        input! {
            n: u8
        };

        *map.entry(n).or_insert(0) += 1;
    }

    if map.len() == 2 && map.values().min().unwrap() == &2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
