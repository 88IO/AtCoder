use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
    };

    let ones = ['H', 'D', 'C', 'S'];
    let seconds = ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

    let mut sv = BTreeSet::new();

    for _ in 0..n {
        input! {
            s: String
        };

        let cs = s.chars().collect::<Vec<char>>();

        if !ones.contains(&cs[0]) {
            println!("No");
            return;
        }

        if !seconds.contains(&cs[1]) {
            println!("No");
            return;
        }

        if sv.contains(&s) {
            println!("No");
            return;
        }

        sv.insert(s);
    }

    println!("Yes");
}
