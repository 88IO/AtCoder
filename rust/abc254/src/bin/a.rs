use proconio::input;

fn main() {
    input! {
        n: usize
    };

    println!("{0: >02}", n % 100);
}
