use proconio::input;

fn main() {
    input! {
        w: String
    };

    let mut v = vec![99; (w.len() + 1) / 2];
    if let Some(last) = v.last_mut() {
        *last = w[..=((w.len() + 1) % 2)].parse().unwrap();
    }

    let cnt: usize = v.iter().sum();

    println!("{}", cnt);

    for (i, k) in v.into_iter().enumerate() {
        for n in 1..=k {
            print!("{} ", n * 100usize.pow(i as u32));
        }
    }
    println!();
}
