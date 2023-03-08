use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        _n: usize, s: String
    };

    let mut pos = (0, 0);
    let mut set = HashSet::new();
    set.insert(pos);

    for c in s.chars() {
        match c {
            'R' => pos.0 += 1,
            'L' => pos.0 -= 1,
            'U' => pos.1 += 1,
            'D' => pos.1 -= 1,
            _ => panic!("unexpected character")
        }
        set.insert(pos);
    }

    if set.len() != s.len() + 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
