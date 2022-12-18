use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };

    if s.len() != 8 {
        println!("No");
        return;
    }

    if  s[0] < 'A' || 'Z' < s[0] {
        println!("No");
        return;
    }

    if  s[1] < '1' || '9' < s[1] {
        println!("No");
        return;
    }

    for i in 2..=6 {
        if  s[i] < '0' || '9' < s[i] {
            println!("No");
            return;
        }
    }

    if  s[7] < 'A' || 'Z' < s[7] {
        println!("No");
        return;
    }

    println!("Yes");
}
