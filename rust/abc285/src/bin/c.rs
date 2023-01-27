use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes
    };

    let mut no = 0;

    for si in s.iter() {
        no *= 26;
        no += (si - 'A' as u8) as usize;
    }

    no += (26usize.pow(s.len() as u32) - 1) / 25;

    println!("{}", no);
}
