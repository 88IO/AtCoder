use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    };

    print_pattern(1, m - n + 2, n, &Vec::new());
}

fn print_pattern(s: usize, t: usize, n: usize, v: &Vec<usize>) {
    for i in s..t {
        let vn = [v.clone(), vec![i]].concat();
        if vn.len() == n {
            println!("{}", &vn.iter().map(|vi| vi.to_string()).collect::<Vec<String>>().join(" "));
        } else {
            print_pattern(i + 1, t + 1, n, &vn);
        }
    }
}
