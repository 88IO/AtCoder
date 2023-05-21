use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u8; n]; n], b: [[u8; n]; n]
    };

    let mut v = [true; 4];

    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                v[0] &= b[i][j] == 1;
                v[1] &= b[n - 1 - j][i] == 1;
                v[2] &= b[n - 1 - i][n - 1 - j] == 1;
                v[3] &= b[j][n - 1 - i] == 1;
            }
        }
    }

    if v.iter().any(|&e| e) {
        println!("Yes");
    } else {
        println!("No");
    }
}
