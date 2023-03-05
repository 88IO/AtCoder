use proconio::input;

fn main() {
    input! {
        n: usize
    };

    for _ in 0..n {
        input! {
            a: isize, b: isize
        };

        println!("{}", a + b);
    }
}
