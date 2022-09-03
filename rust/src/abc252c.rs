use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    };

    let mut times = [0usize; 10];

    for i in 0..10u8 {
        let mut map = BTreeMap::new();

        for si in s.iter() {
            *map.entry(si.find((i + 48) as char).unwrap()).or_insert(0) += 1;
        }

        let last = map.iter().max_by_key(|(_, &cnt)| cnt).unwrap();

        times[i as usize] = *last.0 + 10 * (*last.1 - 1);
    }

    println!("{}", times.iter().min().unwrap());
}
