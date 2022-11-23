use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut fact0 = 1f64;
    for i in 2..n {
        fact0 *= i as f64;
    }
    let mut fact1 = fact0.clone();

    let mut v0 = vec![0; n];
    let mut v1: Vec<_> = (1..=n).collect();

    let mut no = 1f64;

    for i in (0..n).rev() {
        input! {
            p: usize
        };

        no += fact0 * (p - v0[p - 1] - 1) as f64;

        for k in (p - 1)..n {
            v0[k] += 1;
        }

        if i != 0 {
            fact0 /= i as f64;
        }
    }

    no -= 2f64;

    for i in (0..n).rev() {
        let p = (no / fact1) as usize;
        no %= fact1;

        print!("{} ", v1[p]);

        v1.remove(p);

        if i != 0 {
            fact1 /= i as f64;
        }
    }
    println!();
}
