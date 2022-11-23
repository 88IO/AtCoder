use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize
    };

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..n {
        input! {
            a: usize, b: usize
        };

        map.entry(a).or_insert(Vec::new()).push(b);
        map.entry(b).or_insert(Vec::new()).push(a);
    }

    let mut stack = VecDeque::new();
    stack.push_back(1);

    let mut set = HashSet::new();
    set.insert(1);

    while let Some(src) = stack.pop_front() {
        if map.contains_key(&src) {
            for dst in map[&src].iter() {
                if !set.contains(dst) {
                    set.insert(*dst);
                    stack.push_front(*dst);
                }
            }
        }
    }

    println!("{}", set.iter().max().unwrap());
}
