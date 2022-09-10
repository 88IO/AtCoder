use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n]
    };

    let mut counter = s.iter().filter(|&&e| e == '1').count();
    let mut counter_max = counter;

    let mut v: Vec<(char, usize)> = s.into_iter().zip(w.into_iter()).collect();
    v.sort_by_key(|e| e.1);

    let mut w_prev = 0;

    for (c, w) in v.into_iter() {
        if w != w_prev && counter > counter_max {
            counter_max = counter;
        }

        if c == '0' {
            counter += 1;
        } else {
            counter -= 1;
        }

        w_prev = w;
    }

    if counter > counter_max {
        counter_max = counter;
    }

    println!("{}", counter_max);
}
