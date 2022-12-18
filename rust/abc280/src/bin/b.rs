use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut s_prev = 0;

    for _ in 0..n {
        input! {
            sk: isize
        };

        print!("{} ", sk - s_prev);
        s_prev = sk;
    }
    println!();
}
