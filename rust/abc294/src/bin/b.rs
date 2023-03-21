use proconio::input;

fn main() {
    input! {
        h: usize, w: usize
    }

    let base = 'A' as u8 - 1;

    for _ in 0..h {
        input! {
            a: [u8; w]
        };

        println!("{}", a.iter().map(|&ai| {
            if ai == 0 {
                '.'
            } else {
                (base + ai) as char
            }
        }).collect::<String>());
    }
}
