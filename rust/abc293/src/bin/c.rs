use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        h: usize, w: usize, a: [[usize; w]; h]
    };

    let set = HashSet::new();

    let mut ans = 0;

    ans += move_to((0, 0), &a, &set);

    println!("{}", ans);
}

fn move_to((i, j): (usize, usize), a: &Vec<Vec<usize>>, s: &HashSet<usize>) -> usize {
    let mut count = 0;
    let mut set = s.clone();

    if set.contains(&a[i][j]) {
        return 0;
    } else if i == a.len() - 1 && j == a[0].len() - 1 {
        return 1;
    } else {
        set.insert(a[i][j]);

        if i != a.len() - 1 {
            count += move_to((i + 1, j), a, &set);
        }
        if j != a[0].len() - 1 {
            count += move_to((i, j + 1), a, &set);
        }

        return count;
    }
}
