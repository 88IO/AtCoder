use std::collections::HashMap;
use proconio::input;

fn main() {
    input! {
        s: String
    };

    let mut map = HashMap::<char, u32>::new();

    for si in s.chars() {
        *map.entry(si).or_insert(0) += 1;
    }

    for (k, v) in map {
        if v == 1 {
            println!("{}", k);
            return;
        }
    }
    println!("-1");
}
