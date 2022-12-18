use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, a: [usize; m]
    };

    let mut b: Vec<_> = (0..=n).collect();
    let mut op: Vec<_> = vec![(0, 0); m];
    let mut rb: Vec<_> = vec![0; n + 1];

    for i in 0..m {
        op[i] = (b[a[i]], b[a[i] + 1]);
        b.swap(a[i], a[i] + 1);
    }

    for i in 1..=n {
        rb[b[i]] = i;
    }

    for i in 0..m {
        match op[i] {
            (1, bi) => {
                println!("{}", rb[bi]);
            },
            (bi, 1) => {
                println!("{}", rb[bi]);
            },
            (_, _) => {
                println!("{}", rb[1]);
            }
        }
    }
}
