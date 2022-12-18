use proconio::input;
use num::Float;

fn main() {
    input! {
        a: f32, b: f32
    };

    let x = Float::cbrt((a / 2f32 / b).powf(2f32)) - 1f32;

    if x < 0f32 {
        println!("{}", a);
    } else {
        let x1 = x.floor();
        let x2 = x.ceil();
        println!("{}", (a / Float::sqrt(1f32 + x1) + b * x1).min(a / Float::sqrt(1f32 + x2) + b * x2));
    }
}
