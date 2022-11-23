use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, m: usize
    };

    let mut v = vec![BTreeSet::new(); n];

    for _ in 0..m {
        input! {
            a: usize, b: usize
        };
        v[a - 1].insert(b);
        v[b - 1].insert(a);
    }

    for vi in v.iter() {
        print!("{}", vi.len());
        for d in vi.iter() {
            print!(" {}", d);
        }
        println!();
    }
}
