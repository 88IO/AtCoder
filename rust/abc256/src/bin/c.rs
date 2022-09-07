use proconio::input;

fn main() {
    input! {
        h: [isize; 3],
        w: [isize; 3]
    };

    let mut counter = 0;

    for e00 in 1..=(h[0] - 2) {
        for e10 in 1..=(h[0] - e00 - 1) {
            let e20 = h[0] - e00 - e10;

            for e01 in 1..=(w[0] - e00 - 1) {
                let e02 = w[0] - e00 - e01;

                for e11 in 1..=(h[1] - e01 - 1).min(w[1] - e10 - 1) {
                    let e21 = h[1] - e01 - e11;
                    let e12 = w[1] - e10 - e11;

                    let e22h = h[2] - e02 - e12;
                    let e22w = w[2] - e20 - e21;

                    if e22h == e22w && e22h > 0 {
                        counter += 1;
                    }
                }
            }
        }
    }

    println!("{}", counter);
}
