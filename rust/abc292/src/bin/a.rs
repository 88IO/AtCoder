use proconio::input;

fn main() {
    input! {
        mut s: String
    };

    s.make_ascii_uppercase();

    println!("{}", s);
}
