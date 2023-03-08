use proconio::input;

fn main() {
    input! {
        n: usize, q: usize
    };

    let mut players = vec![0; n];

    for _ in 0..q {
        input! {
            c: usize, x: usize
        };

        match c {
            1 => players[x - 1] += 1,
            2 => players[x - 1] += 2,
            3 => println!("{}", if players[x - 1] >= 2 { "Yes" } else { "No" }),
            _ => panic!()
        }
    }
}
