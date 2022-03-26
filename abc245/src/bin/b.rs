use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
    };

    let mut vis = vec![false; n];
    for ai in a {
        if ai < n {
            vis[ai] = true;
        }
    }

    for i in 0..n {
        if !vis[i] {
            println!("{}", i);
            return;
        }
    }

    println!("{}", n);
}
