use proconio::input;

fn main() {
    input! {
        n: usize, s: [String; n]
    };

    for si in s.iter().rev() {
        println!("{}", si);
    }
}
