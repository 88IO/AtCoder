use proconio::input;
use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        n: usize, m: usize
    };

    let mut tree = UnionFind::new(n);
    let mut comp = n;

    for _ in 0..m {
        input! {
            u: usize, v: usize
        };

        if tree.equiv(u - 1, v - 1) {
            println!("No");
            return;
        }

        tree.union(u - 1, v - 1);
        comp -= 1;
    }

    if comp == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
