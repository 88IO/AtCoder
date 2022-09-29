use proconio::input;

fn main() {
    let mut find_prev = false;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    for i in 1..=10 {
        input! {
            s: String
        };

        if let Some(p) = s.find('#') {
            if !find_prev {
                a = i;
                c = p + 1;
                d = s.rfind('#').unwrap() + 1;
                find_prev = true;
            }
            b = i;
        }
    }

    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
