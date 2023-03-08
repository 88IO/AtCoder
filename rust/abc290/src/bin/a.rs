use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n], b: [usize; m]
    };

    let mut sum = 0;

    for i in b.iter() {
        sum += a[i - 1];
    }

    println!("{}", sum);
}
