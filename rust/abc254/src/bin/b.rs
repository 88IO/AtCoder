use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut a = Vec::new();

    for i in 1..=n {
        for j in (1..(i - 1)).rev() {
            a[j] = a[j] + a[j - 1];
        }
        a.push(1);

        for aj in a.iter() {
            print!("{} ", aj);
        }
        println!();
    }
}
