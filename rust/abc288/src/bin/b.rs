use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, k: usize
    };

    let mut set = BTreeSet::new();

    for i in 0..n {
        input! {
            s: String
        };

        if i < k {
            set.insert(s);
        }
    }


    for si in set.iter()  {
        println!("{}", si);
    }
}
