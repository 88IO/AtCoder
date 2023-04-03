use proconio::input;

fn main() {
    input! {
        n: usize
    };

    for _ in 0..n {
        input! {
            w: String
        };

        match w.as_str() {
            "and" | "not" | "that" | "the" | "you" => {
                println!("Yes");
                return;
            },
            _ => {
            }
        }
    }

    println!("No");
}
