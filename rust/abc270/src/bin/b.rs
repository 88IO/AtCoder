use proconio::input;

fn main() {
    input! {
        x: isize,
        y: isize,
        z: isize
    };

    if 0 < y && y < x {
        if y < z {
            println!("-1");
        } else {
            println!("{}", z.abs() + (x - z).abs());
        }
    } else if x < y && y < 0 {
        if z < y {
            println!("-1");
        } else {
            println!("{}", z.abs() + (x - z).abs());
        }
    } else {
        println!("{}", x.abs());
    }
}
