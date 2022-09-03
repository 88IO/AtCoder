use proconio::input;

fn main() {
    input! {
        n: isize
    };

    println!("{}", (n % 998244353 + 998244353) % 998244353);
}
