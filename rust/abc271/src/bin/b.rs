use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [[usize]; n]
    };

    for _ in 0..q {
        input! {
            s: usize,
            t: usize
        };

        println!("{}", a[s - 1][t - 1]);
    }
}
