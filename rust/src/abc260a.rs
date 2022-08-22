use std::collections::HashMap;
use proconio::input;

fn main() {
    input! {
        S: String
    };

    let mut map = HashMap::<char, u32>::new();

    for s in S.chars() {
        *map.entry(s).or_insert(0) += 1;
    }

    for (k, v) in map {
        if v == 1 {
            println!("{}", k);
            return;
        }
    }
    println!("-1");
}
