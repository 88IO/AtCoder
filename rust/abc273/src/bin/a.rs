use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    println!("{}", result);
}
