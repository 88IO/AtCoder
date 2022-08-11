use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut counter: usize = 0;
    let mut ieqv: usize = 0;

    for (i, &ai) in a.iter().enumerate() {
        if i + 1 < ai && i + 1 == a[ai - 1] {
            counter += 1;
        } else if i + 1 == ai {
            ieqv += 1;
        }
    }

    counter += ieqv * (ieqv - 1) / 2;

    println!("{}", counter);
}

