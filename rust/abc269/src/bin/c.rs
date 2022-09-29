use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let s = format!("{:b}", n);

    let v: Vec<_> =  s.rmatch_indices('1').collect();

    for i in 0..2usize.pow(v.len() as u32) {
        let x = format!("{:b}", i).chars().rev().enumerate().fold(0, |acc, (j, c)| {
            if c == '1' {
                acc + 2usize.pow((s.len() - v[j].0 - 1) as u32)
            } else {
                acc
            }
        });
        println!("{}", x);
    }
}
