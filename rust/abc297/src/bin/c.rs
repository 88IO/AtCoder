use proconio::input;

fn main() {
    input! {
        h: usize, _w: usize
    };

    for _ in 0..h {
        input! {
            s: String
        };

        println!("{}", s.replace("TT", "PC"));
    }
}
