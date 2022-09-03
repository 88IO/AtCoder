use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: u64,
        Y: u64
    };

    let mut red: u64 = 1;
    let mut blue: u64 = 0;
    let mut tmp;

    for _ in 1..N {
        tmp = red * X * Y + blue * Y;
        red = red * (1 + X) + blue;
        blue = tmp;
    }

    println!("{}", blue);
}
