use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    };

    let mut sv = Vec::new();
    let mut tv = Vec::new();

    for si in s.into_iter() {
        if let Some((c, k)) = sv.last_mut() {
            if si == *c {
                *k += 1;
                continue;
            }
        }
        sv.push((si, 1));
    }
    for ti in t.into_iter() {
        if let Some((c, k)) = tv.last_mut() {
            if ti == *c {
                *k += 1;
                continue;
            }
        }
        tv.push((ti, 1));
    }

    if sv.len() == tv.len() && sv.iter().zip(tv.iter()).all(|(s, t)| {
        s.0 == t.0 && (s.1 == t.1 || s.1 < t.1 && s.1 >= 2)
    }) {
        println!("Yes")
    } else {
        println!("No")
    }
}
