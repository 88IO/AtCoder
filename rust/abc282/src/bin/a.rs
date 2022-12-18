use proconio::input;

fn main() {
    input! {
        k: u8
    };

    for i in 0..k {
        print!("{}", ('A' as u8 + i) as char);
    }
    println!();
}
