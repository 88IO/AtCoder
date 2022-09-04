use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n]
    };

    let mut b: isize = (1..=m as isize).zip(a[0..m].iter()).map(|(i, bi)| i * bi).sum();
    let mut best = b;
    let mut s: isize = a[0..m].iter().sum();

    for i in m..n {
        b += a[i] * m as isize - s;
        s += a[i] - a[i - m];
        if b > best { best = b; }
    }

    println!("{}", best);
}
