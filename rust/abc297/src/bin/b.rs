use proconio::input;

fn main() {
    input! {
        s: String
    };

    let b0 = s.find('B').unwrap();
    let b1 = s.rfind('B').unwrap();
    let r0 = s.find('R').unwrap();
    let r1 = s.rfind('R').unwrap();
    let k = s.find('K').unwrap();

    if (b0 + b1) % 2 == 1 && (r0 < k && k < r1) {
        println!("Yes");
    } else {
        println!("No");
    }
}
