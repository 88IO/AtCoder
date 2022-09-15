use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    };

    a.sort();

    let mut aa: Vec<(usize, usize)> = Vec::new();

    for ai in a.into_iter() {
        match aa.last_mut() {
            Some((e, c)) if *e == ai => {
                *c += 1;
            },
            _ => {
                aa.push((ai, 1))
            }
        }
    }

    let mut prev_cnt = 0;
    let mut result = 0;

    for (_, c) in aa.iter() {
        result += prev_cnt * c * (n - c - prev_cnt);
        prev_cnt += c;
    }

    println!("{}", result);
}
