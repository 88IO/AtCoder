use proconio::input;
use proconio::marker::Chars;

fn main() {
    for i in (1..=8).rev() {
        input! {
            s: Chars
        };

        for j in 0..8u8 {
            if s[j as usize] == '*' {
                println!("{}{}", ('a' as u8 + j) as char, i);
            }
        }
    }
}
