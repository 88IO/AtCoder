use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k]
    };

    for _ in 0..q {
        input! {
            l: usize
        };

        if a[l - 1] != n && (l == k || a[l] != a[l - 1] + 1) {
            a[l - 1] += 1;
        }
    }

    for i in a.iter() {
        print!("{} ", i);
    }
}
