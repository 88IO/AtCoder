use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut v = vec![false; n];

    for i in 0..n {
        input! {
            ai: usize
        };

        if !v[i] {
            v[ai - 1] = true;
        }
    }

    let ans: Vec<(usize, &bool)> = v.iter().enumerate().filter(|(_i, &vi)| !vi).collect();
    println!("{}", ans.len());

    for (ai, _vi) in ans.iter() {
        print!("{} ", ai + 1);
    }
    println!();
}
