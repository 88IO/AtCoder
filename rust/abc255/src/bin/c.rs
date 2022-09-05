use proconio::input;

fn main() {
    input! {
        x: isize,
        a: isize,
        d: isize,
        n: isize
    };

    if d == 0 {
        println!("{}", (x - a).abs());
        return;
    }

    let p = (x - a) / d;
    let q = (x - a) % d;
    //println!("p={}, q={}", p, q);

    if p < 0 {
        println!("{}", (x - a).abs());
    } else if p < n {
        println!("{}", (q.abs()).min((q - d).abs()));
    } else {
        println!("{}", (x - a - d * (n - 1)).abs());
    }
}
