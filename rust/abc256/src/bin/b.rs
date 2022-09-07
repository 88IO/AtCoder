use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut base = Vec::new();
    let mut p = 0;

    for _ in 0..n {
        input! {
            a: usize
        };

        base.push(0);

        for i in (0..base.len()).rev() {
            base[i] += a;
            if base[i] >= 4 {
                p += 1;
                base.remove(i);
            }
        }
    }

    println!("{}", p);
}
