use proconio::input;

fn main() {
    input! {
        a: usize, b: usize
    };

    if 2 * a == b || 2 * a + 1 == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
