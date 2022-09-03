use proconio::input;

fn main() {
    input! {
        a: (isize, isize),
        b: (isize, isize),
        c: (isize, isize),
        d: (isize, isize)
    };

    if outer_product((d.0 - a.0, d.1 - a.1), (b.0 - a.0, b.1 - a.1)) > 0 {
        println!("No");
        return;
    }
    if outer_product((a.0 - b.0, a.1 - b.1), (c.0 - b.0, c.1 - b.1)) > 0 {
        println!("No");
        return;
    }
    if outer_product((b.0 - c.0, b.1 - c.1), (d.0 - c.0, d.1 - c.1)) > 0 {
        println!("No");
        return;
    }
    if outer_product((c.0 - d.0, c.1 - d.1), (a.0 - d.0, a.1 - d.1)) > 0 {
        println!("No");
        return;
    }

    println!("Yes");
}

fn outer_product(a: (isize, isize), b: (isize, isize)) -> isize {
    a.0 * b.1 - a.1 * b.0
}
