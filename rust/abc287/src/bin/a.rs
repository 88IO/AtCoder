use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut n_for = 0;

    for _ in 0..n {
        input! {
            s: String
        };

        if s == "For" {
            n_for += 1;
        }
    }

    if n_for > n / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
