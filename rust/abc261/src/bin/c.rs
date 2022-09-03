use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut map: HashMap<String, usize> = HashMap::new();

    for _ in 0..n {
        input! {
            s: String
        };

        print!("{}", s);

        map.entry(s)
            .and_modify(|e| {
                println!("({})", *e);
                *e += 1;
            })
            .or_insert_with(|| {
                println!("");
                1
            });
    }
}
