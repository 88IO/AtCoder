use proconio::input;

fn main() {
    input! {
        _n: usize, s: String
    };

    let good = s.find('o');
    let bad = s.find('x');

    if good.is_some() && bad.is_none() {
        println!("Yes");
    } else {
        println!("No");
    }
}
