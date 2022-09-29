use proconio::input;
use std::rc::Rc;

struct Node {
    value: usize,
    prev: Option<Rc<Node>>
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize
    };

    let mut path = vec![Vec::new(); n];
    let mut queue = Vec::new();

    for _ in 1..n {
        input! {
            u: usize,
            v: usize
        };

        path[u - 1].push(v);
        path[v - 1].push(u);
    }

    queue.push(Rc::new(Node { value: y, prev:None }));

    while let Some(node) = queue.pop() {
        for b in path[node.value - 1].iter() {
            if node.prev.is_some() && node.prev.as_ref().unwrap().value == *b {
                continue;
            }

            let mut new = Rc::new(Node { value: *b, prev: Some(node.clone()) });

            if new.value == x {
                while let Some(n) = new.prev.clone() {
                    print!("{} ", new.value);
                    new = n;
                }
                println!("{}", new.value);
                return;
            }

            queue.push(new);
        }
    }
}
