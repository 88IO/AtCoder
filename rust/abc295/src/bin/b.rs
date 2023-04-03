use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: i8, c: i8,
        b: [Chars; r]
    }

    let mut ans = b.clone();

    for x in 0..r {
        for y in 0..c {
            let n = b[x as usize][y as usize] as i8 - '0' as i8;
            if 1 <= n && n <= 9 {
                explode((x, y), (r, c), n, &mut ans);
            }
        }
    }

    for x in 0..r {
        println!("{}",
                 ans[x as usize].iter().collect::<String>());
    }
}

fn explode((x, y): (i8, i8), (r, c): (i8, i8), n: i8, ans: &mut Vec<Vec<char>>) {
    for i in 0..=n {
        for j in 0..=(n - i) {
            ans[(x + i).min(r - 1) as usize][(y + j).min(c - 1) as usize] = '.';
            ans[(x - i).max(0) as usize][(y + j).min(c - 1) as usize] = '.';
            ans[(x + i).min(r - 1) as usize][(y - j).max(0) as usize] = '.';
            ans[(x - i).max(0) as usize][(y - j).max(0) as usize] = '.';
        }
    }
}
