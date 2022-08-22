use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut X: usize,
        mut Y: usize,
        mut Z: usize,
    };
    input! {
        A: [usize; N], B: [usize; N]
    };

    let mut x = A.iter().enumerate().collect::<Vec<(_, _)>>();
    x.sort_by(|x, y| (y.1).partial_cmp(&(x.1)).unwrap());

    let mut y = B.iter().enumerate().collect::<Vec<(_, _)>>();
    y.sort_by(|x, y| (y.1).partial_cmp(&(x.1)).unwrap());

    let mut C = vec![0; N];
    for i in 0..N {
        C[i] = A[i] + B[i];
    }

    let mut z = C.iter().enumerate().collect::<Vec<(_, _)>>();
    z.sort_by(|x, y| (y.1).partial_cmp(&(x.1)).unwrap());

    let mut pass = vec![false; N];

    for xi in x.iter() {
        if X == 0 { break; }
        if !pass[xi.0] {
            pass[xi.0] = true;
            X -= 1;
        }
    }

    for yi in y.iter() {
        if Y == 0 { break; }
        if !pass[yi.0] {
            pass[yi.0] = true;
            Y -= 1;
        }
    }

    for zi in z.iter() {
        if Z == 0 { break; }
        if !pass[zi.0] {
            pass[zi.0] = true;
            Z -= 1;
        }
    }

    for i in 0..N {
        if pass[i] {
            println!("{}", i + 1);
        }
    }
}
