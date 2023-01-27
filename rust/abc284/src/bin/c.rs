use proconio::input;
use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        n: usize, m: usize
    };

    let mut tree: UnionFind<usize> = UnionFind::new(n + 1);

    let mut comp = n;

    for _ in 0..m {
        input! {
            u: usize, v: usize
        };

        if !tree.equiv(u, v) {
            comp -= 1;
        }

        tree.union(u, v);
    }

    println!("{}", comp);
}
