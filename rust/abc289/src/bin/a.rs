use proconio::input;

fn main() {
    input! {
        s: String
    };

    let fliped: String = s.chars().map(|c| {
        match c {
            '0' => '1',
            '1' => '0',
            _ => panic!("unexpected value")
        }
    }).collect();

    println!("{}", fliped);
}
