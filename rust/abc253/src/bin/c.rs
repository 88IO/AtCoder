use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input! {
        q: usize
    };

    let mut map = BTreeMap::new();

    for _ in 0..q {
        input! {
            n: usize
        };

        match n {
            1 => {
                input! {
                    x: usize
                };
                *map.entry(x).or_insert(0) += 1;
            },
            2 => {
                input! {
                    x: usize,
                    c: usize
                };
                if let Some(e) = map.get_mut(&x) {
                    if *e > c {
                        *e -= c;
                    } else {
                        map.remove(&x);
                    }
                }
            },
            3 => {
                let first = *map.keys().next().unwrap();
                let last = *map.keys().last().unwrap();
                println!("{}", last.wrapping_sub(first));
            },
            _ => {
                panic!("unexpected value");
            }
        }
    }
}
