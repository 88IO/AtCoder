use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1]
    };
    input! {
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2]
    };

    let hs = h1 - h2;
    let ws = w1 - w2;

    for combh in (0..h1).combinations(hs) {
        let mut ah = a.clone();
        for (i, v) in combh.iter().enumerate() {
            ah.remove(v - i);
        }

        for combw in (0..w1).combinations(ws) {
            let mut aw = ah.clone();
            for (i, v) in combw.iter().enumerate() {
                for h in aw.iter_mut() {
                    h.remove(v - i);
                }
            }

            if aw == b {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
