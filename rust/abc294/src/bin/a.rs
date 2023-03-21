use proconio::input;

fn main() {
    input! {
        n: usize, a: [usize; n]
    };

    for even in a.into_iter().filter(|&x| x % 2 == 0) {
        print!("{} ", even);
    }
    println!();
}
