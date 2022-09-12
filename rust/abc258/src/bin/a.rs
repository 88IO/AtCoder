use proconio::input;

fn main() {
    input! {
        k: usize
    };

    println!("{0:>02}:{1:>02}", (21 + k / 60) % 24, k % 60);
}
