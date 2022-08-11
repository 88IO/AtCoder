use proconio::input;

fn main() {
    input! {
        mut n: usize,
        p: [usize; n - 1]
    };

    let mut counter = 1;

    while p[n - 2] != 1 {
        n = p[n - 2];
        counter += 1;
    }

    println!("{}", counter);
}
