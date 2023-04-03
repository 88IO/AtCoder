use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, x: isize
    };

    let mut set = BTreeSet::new();

    for _ in 0..n {
        input! {
            a: isize
        };
        set.insert(a);
    }

    for a in set.iter() {
        if set.contains(&(x + a)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
