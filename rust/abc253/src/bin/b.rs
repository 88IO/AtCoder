use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    };

    let mut pos = [(0isize, 0isize), (0isize, 0isize)];
    let mut index = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                pos[index] = (i as isize, j as isize);
                index += 1;
            }
        }
    }

    println!("{}", (pos[0].0 - pos[1].0).abs() + (pos[0].1 - pos[1].1).abs());
}
