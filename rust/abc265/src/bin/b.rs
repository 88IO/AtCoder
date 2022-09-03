use std::collections::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: isize,
        a: [usize; n - 1],
        xy: [(usize, usize); m],
    };

    let map: HashMap<usize, usize> = xy.into_iter().collect();

    for i in 1..n {
        t -= a[i - 1] as isize;

        if t <= 0 {
            println!("No");
            return;
        }

        if let Some(&y) = map.get(&(i + 1)) {
            t += y as isize;
        }
    }

    println!("Yes");
}
