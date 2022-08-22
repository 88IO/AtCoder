use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    // index -> valueの配列
    // 可読性のため、0は使わない
    let mut arr: Vec<usize> = (0..=n).collect();
    // value -> indexの配列
    let mut rarr = arr.clone();

    for _ in 0..q {
        input! {
            x: usize
        };
        // x_iのインデクス
        let idx = rarr[x];
        // 入れ替え先のインデクス
        let next_idx = if idx == n { idx - 1 } else { idx + 1 };
        // 入れ替え先の要素
        let v = arr[next_idx];
        // ボールの入れ替え
        arr.swap(idx, next_idx);
        rarr.swap(x, v);
    }

    for i in 1..=n {
        print!("{} ", arr[i]);
    }
    println!("");
}
