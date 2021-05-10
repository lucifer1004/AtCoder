#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p: usize,
        a: usize,
        b: usize,
    }

    let mut v = vec![vec![1]];
    let mut u = a;
    let mut vis = vec![false; p];
    vis[1] = true;
    while !vis[u] {
        v.push(vec![u]);
        vis[u] = true;
        u = u * a % p;
    }
    let n = v.len();
    for i in 0..n {
        let mut u = v[i][0] * b % p;
        while !vis[u] {
            v[i].push(u);
            u = u * b % p;
        }
    }

    let m = v[0].len();
    if n * m < p - 1 {
        println!("No");
    } else {
        println!("Yes");
        for i in 0..m {
            print!("{} ", v[0][i]);
        }
        for i in 0..m {
            if i % 2 == 0 {
                for j in 1..n {
                    print!("{} ", v[j][m - 1 - i]);
                }
            } else {
                for j in (1..n).rev() {
                    print!("{} ", v[j][m - 1 - i]);
                }
            }
        }
        println!("1");
    }
}