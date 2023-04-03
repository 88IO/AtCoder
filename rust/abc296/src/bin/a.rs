use proconio::input;

fn main() {
    input! {
        _n: usize, s: String
    };

    let mut prev = '0';

    for c in s.chars() {
        if c == prev {
            println!("No");
            return;
        }
        prev = c;
    }

    println!("Yes");
}
