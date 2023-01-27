use proconio::input;

fn main() {
    input! {
        _n: usize, s: String
    };

    println!("{}", s.replace("na", "nya"));
}
