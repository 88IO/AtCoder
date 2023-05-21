use proconio::input;

fn main() {
    input! {
        _n: usize, s: String
    };

    let mut pipe_n = 0;

    for c in s.chars() {
        if c == '|' {
            pipe_n += 1;
        } else if c == '*' && pipe_n == 1 {
            println!("in");
            return;
        }
    }
    println!("out");
}
