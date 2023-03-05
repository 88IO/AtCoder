use proconio::input;
use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        n: usize, m: usize
    };

    let mut graph = UnionFind::new(n);
    let mut counter = 0;

    for _ in 0..m {
        input! {
            a: usize, b: usize
        };

        if graph.equiv(a - 1, b - 1) {
            counter += 1;
        } else {
            graph.union(a - 1, b - 1);
        }
    }

    println!("{}", counter);
}
