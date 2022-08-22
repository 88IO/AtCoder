use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    };

    for i in 0..n {
        for j in (i + 1)..n {
            if a[i][j] != reverse(a[j][i]) {
                println!("incorrect");
                return;
            }
        }
    }
    println!("correct");
}

fn reverse(c: char) -> char {
    match c {
        'W' => 'L',
        'L' => 'W',
        'D' => 'D',
        _ => panic!("unexpected character."),
    }
}
