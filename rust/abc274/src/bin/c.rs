use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut ameba = vec![0; 2 * n + 1];

    for i in 1..=n {
        input! {
            a: usize
        };

        ameba[2 * i - 1] = ameba[a - 1] + 1;
        ameba[2 * i] = ameba[a - 1] + 1;
    }

    for a in ameba.iter() {
        println!("{}", a);
    }
}
