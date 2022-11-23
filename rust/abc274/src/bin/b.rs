use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize, w: usize,
    };

    let mut boxes = vec![0; w];

    for _ in 0..h {
        input! {
            c: Chars
        }

        for (j, cj) in c.into_iter().enumerate() {
            if cj == '#' {
                boxes[j] += 1;
            }
        }
    }

    for b in boxes.iter() {
        print!("{} ", b);
    }
    println!();
}
