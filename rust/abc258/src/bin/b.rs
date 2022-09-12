use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: isize,
        a: [Bytes; n]
    };

    let mut result = 0;
    let pos = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for i in 0..n {
        for j in 0..n {
            for p in pos.iter() {
                let mut x = i;
                let mut y = j;
                let mut result_tmp = a[x as usize][y as usize] as usize - 48;

                for _ in 0..(n - 1) {
                    x = (n + x + p.0) % n;
                    y = (n + y + p.1) % n;
                    result_tmp = result_tmp * 10 + a[x as usize][y as usize] as usize - 48;
                }
                if result_tmp > result {
                    result = result_tmp;
                }
            }
        }
    }

    println!("{}", result);
}
