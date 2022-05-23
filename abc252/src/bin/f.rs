use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        l: i64,
        a: [i64; n],
    }

    let mut sum = 0i64;
    for &ai in a.iter() {
        sum += ai;
    }

    let mut ans = 0i64;
    let mut pq = a.into_iter().map(|x| -x).collect::<BinaryHeap<_>>();
    if l > sum {
        pq.push(sum - l);
    }
    while pq.len() > 1 {
        let a = pq.pop().unwrap();
        let b = pq.pop().unwrap();
        pq.push(a + b);
        ans -= a + b;
    }

    println!("{}", ans);
}
