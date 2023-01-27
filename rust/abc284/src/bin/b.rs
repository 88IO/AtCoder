use proconio::input;

fn main() {
    input! {
        t: usize
    };

    for _ in 0..t {
        input! {
            n: usize
        };

        let mut counter = 0;

        for _ in 0..n {
            input! {
                a: usize
            };

            counter += a % 2;
        }

        println!("{}", counter);
    }
}
