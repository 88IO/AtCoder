use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, mut a: [usize; n], q: usize
    };

    let mut map = HashMap::new();
    let mut base: isize = -1;

    for _ in 0..q {
        input! {
            t: usize
        };

        match t {
            1 => {
                input! {
                    x: isize
                };
                base = x;
                map.clear();
            },
            2 => {
                input! {
                    i: usize, x: usize
                };
                *map.entry(i).or_insert(if base < 0 {a[i - 1]} else {base as usize}) += x;
            },
            3 => {
                input! {
                    i: usize
                };
                map.entry(i).or_insert(if base < 0 {a[i - 1]} else {base as usize});
                println!("{}", map[&i]);
            },
            _ => {
                panic!("unexpected query.");
            }
        }
    }
}
