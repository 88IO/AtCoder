use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        s: [usize; n], t: [usize; m]
    };

    let mut counter = 0;

    for si in s.iter() {
        let si = si % 1000;
        for ti in t.iter() {
            if si == *ti {
                counter += 1;
                break;
            }
        }
    }

    println!("{}", counter);
}
