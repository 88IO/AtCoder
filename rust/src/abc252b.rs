use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; k]
    };

    let best = *a.iter().max().unwrap();

    for bi in b {
        if a[bi - 1] == best {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
