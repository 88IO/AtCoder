use std::collections::{VecDeque, HashSet};
use proconio::input;

fn main() {
    input! {
        s: String
    };

    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    let mut set: HashSet<String> = HashSet::new();

    queue.push_back((s, 0));

    loop {
        let (ss, n) = queue.pop_front().unwrap();

        if set.contains(&ss) {
            continue;
        }
        set.insert(ss.clone());

        if ss == "atcoder" {
            println!("{}", n);
            return;
        }

        for i in 0..6 {
            let mut v: Vec<char> = ss.chars().collect();
            v.swap(i, i + 1);
            queue.push_back((v.iter().collect(), n + 1));
        }
    }
}
