use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    };

    let max = h.iter().enumerate().max_by_key(|(_i, &e)| e).unwrap();

    println!("{}", max.0 + 1);
}
