use proconio::input;

fn main() {
    input! {
        s: String
    };

    let matches: Vec<_> = s.rmatch_indices('a').collect();

    if matches.is_empty() {
        println!("-1");
    } else {
        println!("{}", matches[0].0 + 1);
    }
}
