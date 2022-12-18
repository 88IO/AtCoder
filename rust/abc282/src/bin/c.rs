use proconio::input;

fn main() {
    input! {
        _n: usize, s: String
    };

    let mut rep = true;

    for c in s.chars() {
        if c == '"' {
            rep = !rep;
        }

        if rep && c == ',' {
            print!(".");
        } else {
            print!("{}", c);
        }
    }
    println!();
}
