use proconio::input;

fn main() {
    input! {
        n: usize, mut x: [usize; 5 * n]
    };

    x.sort();

    let s: f32 = x.iter().skip(n).take(3 * n).sum::<usize>() as f32;

    println!("{}", s / (3. * n as f32));
}
