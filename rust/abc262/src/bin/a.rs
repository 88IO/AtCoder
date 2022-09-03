use proconio::input;

fn main() {
    input! {
        y: isize
    };

    println!("{}", y + (4 - (y - 2) % 4) % 4);
}
