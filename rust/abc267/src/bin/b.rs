use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };

    let s: Vec<usize> = s.iter().map(|&e| e as usize - 48).collect();

    if s[0] != 0 {
        println!("No");
        return;
    }

    let a = [s[6], s[3], s[1] + s[7], s[4], s[2] + s[8], s[5], s[9]];

    for i in 0..5 {
        if a[i] == 0 { continue; }
        for j in (i + 1)..6 {
            if a[j] != 0 { continue; }
            for k in (j + 1)..7 {
                if a[k] != 0 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
