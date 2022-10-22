use proconio::input;
use std::isize;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n]
    };

    a.sort();

    // (odd + odd, even + even)
    let mut sum = ([-10isize.pow(9) - 1; 2], [-10isize.pow(9) - 1; 2]);
    let mut index = (0, 0);

    for k in a.into_iter().rev() {
        if index.0 < 2 && k % 2 == 0 {
            sum.0[index.0] = k;
            index.0 += 1;
        } else if index.1 < 2 && k % 2 == 1 {
            sum.1[index.1] = k;
            index.1 += 1;
        }
    }

    let max_val = (sum.0[0] + sum.0[1]).max(sum.1[0] + sum.1[1]);

    if max_val < 0 {
        println!("-1");
    } else {
        println!("{}", max_val);
    }
}
