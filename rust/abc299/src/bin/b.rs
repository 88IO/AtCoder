use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, t: usize,
        c: [usize; n], r: [usize; n]
    };

    let mut map = HashMap::new();

    for i in 0..n {
        map.entry(c[i]).or_insert(Vec::new()).push((r[i], i + 1));
    }

    if let Some(s) = map.get(&t) {
        println!("{}", s.iter().max().unwrap().1);
    } else {
        println!("{}", map[&c[0]].iter().max().unwrap().1);
    }
}
