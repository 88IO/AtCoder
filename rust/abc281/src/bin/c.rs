use proconio::input;

fn main() {
    input! {
        n: usize, mut t: usize
    };

    let mut s = vec![0; n + 1];

    for i in 1..=n {
        input! {
            a: usize
        };
        s[i]  = s[i - 1] + a;
    }

    t %= s[n];

    for i in 1..=n {
        if s[i] > t {
            println!("{} {}", i, t - s[i - 1]);
            return;
        }
    }
}
