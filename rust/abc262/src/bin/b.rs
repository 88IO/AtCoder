use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
    };

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut counter = 0;

    for _ in 0..m {
        input! {
            u: usize,
            v: usize
        };

        map.entry(u).or_insert(Vec::new()).push(v);
    }

    for bs in map.values() {
        if bs.len() == 1 {
            continue;
        }
        for b in bs {
            if let Some(cs) = map.get(&b) {
                for c in cs {
                    if bs.contains(c) {
                        counter += 1;
                    }
                }
            }
        }
    }

    println!("{}", counter);
}
