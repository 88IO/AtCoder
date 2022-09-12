use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    };
    let mut result = std::usize::MAX;
    let mut before_k = 0;

    for k in 1..=n.min(x) {
        input! {
            a: usize,
            b: usize
        };

        before_k += a + b;

        let time = before_k + b * (x - k);

        if time < result {
            result = time;
        }
    }

    println!("{}", result);
}
