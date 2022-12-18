use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize, m: usize
    };

    let mask = (1 << m) - 1;
    let mut v = vec![0; n];
    let mut counter = 0;

    for i in 0..n {
        input! {
            s: String
        };

        for (j, c) in s.chars().enumerate() {
            if c == 'o' {
                v[i] += 1 << j;
            }
        }
    }

    for comb in (0..n).combinations(2) {
        if v[comb[0]] | v[comb[1]] == mask {
            counter += 1;
        }
    }

    println!("{}", counter);
}
