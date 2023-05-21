use proconio::input;
use std::collections::{HashMap, BTreeSet};

fn main() {
    input! {
        n: usize, q: usize
    };

    let mut boxes = vec![Vec::new(); n + 1];
    let mut cards = HashMap::new();

    for _ in 0..q {
        input! {
            t: usize
        };

        match t {
            1 => {
                input! {
                    i: usize, j: usize
                };
                boxes[j].push(i);
                cards.entry(i).or_insert(BTreeSet::new()).insert(j);
            },
            2 => {
                input! {
                    i: usize
                };
                boxes[i].sort();
                println!("{}", boxes[i].iter().map(|&e| e.to_string()).collect::<Vec<String>>().join(" "));
            },
            3 => {
                input! {
                    j: usize
                };
                println!("{}", cards[&j].iter().map(|&e| e.to_string()).collect::<Vec<String>>().join(" "));
            },
            _ => {
                panic!("unexpected query");
            }
        }
    }
}
