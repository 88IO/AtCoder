use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    };

    let dot = ".".repeat(b);
    let sharp = "#".repeat(b);
    let mut x = String::new();
    let mut y = String::new();

    for i in 0..n {
        if i % 2 == 0 {
            x.push_str(&dot);
            y.push_str(&sharp);
        } else {
            x.push_str(&sharp);
            y.push_str(&dot);
        }
    }

    x.push_str("\n");
    y.push_str("\n");

    let ax = x.repeat(a);
    let ay = y.repeat(a);

    for i in 0..n {
        if i % 2 == 0 {
            print!("{}", ax);
        } else {
            print!("{}", ay);
        }
    }
}
