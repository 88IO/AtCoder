use proconio::input;

fn main() {
    input! {
        s: String
    };

    println!("{}", s.len() + s.chars().filter(|&c| c == 'w').count());
}
