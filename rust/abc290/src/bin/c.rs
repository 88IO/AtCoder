use proconio::input;

fn main() {
    input! {
        n: usize, k: usize, mut a: [usize; n]
    };

    a.sort();
    a.dedup();

    for (i, c) in a.iter().enumerate() {
        if i != *c {
            println!("{}", i);
            return;
        }
        if i == k {
            break;
        }
    }

    println!("{}", k.min(a.len()));
}
