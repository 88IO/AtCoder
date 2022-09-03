use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize
    };

    println!("{}", (r != 1) as usize + (r != h) as usize + (c != 1) as usize + (c != w) as usize);
}
