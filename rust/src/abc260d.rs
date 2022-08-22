use proconio::input;
use std::collections::BTreeMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        P: [usize; N]
    };

    let mut result: Vec<isize> = vec![-1; N];
    let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for i in 0..N {
        if let Some((&k, _)) = map.range(P[i]..).nth(0) {
            let mut v = map.remove(&k).unwrap();
            v.push(P[i]);
            if v.len() == K {
                for pi in v.iter() {
                    result[pi - 1] = (i + 1) as isize;
                }
            } else {
                map.insert(P[i], v);
            }
        } else {
            if K == 1 { result[P[i] - 1] = (i + 1) as isize; }
            else { map.insert(P[i], vec![P[i]]); }
        }
    }

    for i in 0..N {
        println!("{}", result[i]);
    }
}
