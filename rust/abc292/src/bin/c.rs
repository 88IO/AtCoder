use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut c = vec![0; n];
    let mut ans = 0;

    for i in 1..n {
        let sq = (i as f32).sqrt();
        for j in 1..=(sq as usize) {
            if i % j == 0 {
                c[i] += 1;
            }
        }
        c[i] *= 2;
        if sq == sq.floor() {
            c[i] -= 1;
        }
    }

    for i in 1..=((n - 1) / 2) {
        ans += c[i] * c[n - i];
    }
    ans *= 2;
    if n % 2 == 0 {
        ans += c[n / 2] * c[n / 2];
    }

    println!("{}", ans);
}
