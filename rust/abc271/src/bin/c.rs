use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    };

    a.sort();
    a.dedup();
    let mut dec: VecDeque<_> = a.into_iter().collect();

    let mut read = n - dec.len();
    let mut now = 0;


    loop {
        match dec.front() {
            Some(&new) if new == now + 1 => {
                now = dec.pop_front().unwrap();
            },
            _ => {
                if read < 2 {
                    for _ in 0..(2 - read) {
                        match dec.pop_back() {
                            Some(_) => { read += 1; },
                            None => {
                                println!("{}", now);
                                return;
                            }
                        }
                    }
                }
                now += 1;
                read -= 2;
            }
        }
    }
}
