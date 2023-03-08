use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, a: [usize; m]
    };

    let mut stack = Vec::new();

    for i in 1..=n {
        if a.contains(&i) {
            stack.push(i);
        } else {
            print!("{} ", i);
            while let Some(j) = stack.pop() {
                print!("{} ", j);
            }
        }
    }
    println!();
}
