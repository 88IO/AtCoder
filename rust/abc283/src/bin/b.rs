use proconio::input;

fn main() {
    input! {
        n: usize, mut a: [usize; n], q: usize
    };

    for _ in 0..q {
        input! {
            t: u8
        };

        match t {
            1 => {
                input! {
                    k: usize, x: usize
                };
                a[k - 1] = x;
            },
            2 => {
                input! {
                    k: usize
                };
                println!("{}", a[k - 1]);
            },
            _ => {
                panic!("unexpeced type.");
            }
        }
    }
}
