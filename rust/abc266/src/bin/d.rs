use proconio::input;

fn main() {
    input! {
        n: isize
    };

    let mut dp = [[0; 5]; 2];
    let mut head: usize = 0;
    let mut t_prev = 0;

    for _ in 0..n {
        input! {
            t: isize,
            x: usize,
            a: usize
        };

        let range = 4.min(t - t_prev);

        for j in 0..5 {
            let l = 0.max(j - range) as usize;
            let r = 4.min(j + range) as usize;
            dp[(head + 1) % 2][j as usize] = *dp[head][l..=r].iter().max().unwrap();
        }

        if t as usize >= x {
            dp[(head + 1) % 2][x] += a;
        }

        t_prev = t;
        head = (head + 1) % 2;
    }

    println!("{}", dp[head].iter().max().unwrap());
}
