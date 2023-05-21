use proconio::input;

fn main() {
    input! {
        _n: usize, s: String
    };

    let mut v: Vec<(char, usize)> = Vec::new();

    let mut prev_char = ' ';
    for c in s.chars() {
        if c == prev_char {
            v.last_mut().unwrap().1 += 1;
        } else {
            v.push((c, 1));
        }
        prev_char = c;
    }

    let mut max_n = 0;

    if v.len() == 1 {
        println!("-1");
    } else {
        for vi in v.iter() {
            if vi.0 == 'o' && vi.1 > max_n {
                max_n = vi.1;
            }
        }
        println!("{}", max_n);
    }
}
