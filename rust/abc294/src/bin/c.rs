use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        mut a: [usize; n], mut b: [usize; m]
    };

    let mut ni = 0;
    let mut mi = 0;

    for i in 1..=(n + m) {
        if ni != n && (mi == m || a[ni] < b[mi]) {
            a[ni] = i;
            ni += 1;
        } else {
            b[mi] = i;
            mi += 1;
        }
    }

    println!("{}", a.iter().map(|&ai| ai.to_string()).collect::<Vec<String>>().join(" "));
    println!("{}", b.iter().map(|&bi| bi.to_string()).collect::<Vec<String>>().join(" "));
}
