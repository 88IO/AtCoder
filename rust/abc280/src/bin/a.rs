use proconio::input;

fn main() {
    input! {
        h: usize, _w: usize
    };

    let mut counter = 0;

    for _ in 0..h {
        input! {
            s: String
        };

        counter += s.chars().filter(|&c| c == '#').count();
    }

    println!("{}", counter);
}
