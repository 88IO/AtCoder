use proconio::input;

fn main() {
    input! {
        a: f32,
        b: f32
    };

    println!("{:.3}", (b / a * 1000f32).round() / 1000f32);
}
