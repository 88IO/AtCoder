use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(isize, isize); n]
    };

    let dist_square = xy.iter()
        .map(|(x, y)| {
            a.iter().map(|&ai| {
                let (ax, ay) = xy[ai - 1];
                (ax - x).pow(2) + (ay - y).pow(2)
            }).min().unwrap()
        }).max().unwrap();

    println!("{}", (dist_square as f64).sqrt());
}
