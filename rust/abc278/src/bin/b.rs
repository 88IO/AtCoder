use proconio::input;

fn main() {
    input! {
        mut h: usize, mut m: usize
    };

    loop {
        let mh = h / 10 * 10 + m / 10;
        let mm = h % 10 * 10 + m % 10;

        if mh < 24 && mm < 60 {
            println!("{} {}", h, m);
            return;
        }

        m = (m + 1) % 60;
        if m == 0 {
            h = (h + 1) % 24;
        }
    }
}
