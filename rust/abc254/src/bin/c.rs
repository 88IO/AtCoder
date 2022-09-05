use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    };

    if k == 1 {
        println!("Yes");
        return;
    }

    let mut ans = a.clone();
    ans.sort();

    let mut b: Vec<Vec<usize>> = vec![Vec::new(); k];

    for i in 0..n {
        b[i % k].push(a[i]);
    }

    for bi in b.iter_mut() {
        bi.sort_by_key(|&e| cmp::Reverse(e));
    }

    for i in 0..n {
        a[i] = b[i % k].pop().unwrap();
    }

    if a == ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

