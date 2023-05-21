use proconio::input;

fn main() {
    input! {
        mut a: usize, mut b: usize
    };

    let mut counter = 0;

    while a != b {
        if a == 0 || b == 0 {
            counter -= 1;
            break;
        } else if a > b {
            counter += a / b;
            a %= b;
        } else {
            counter += b / a;
            b %= a;
        }
    }

    println!("{}", counter);
}
