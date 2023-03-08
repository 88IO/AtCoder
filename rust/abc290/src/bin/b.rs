use proconio::input;

fn main() {
    input! {
        _n: usize, k: usize,
        s: String
    };

    let mut o = 0;

    let t: String = s.chars().map(|c| {
        if o == k {
            return 'x'
        }
        if c == 'o' {
            o += 1;
        }
        c
    }).collect();

    println!("{}", t);
}
