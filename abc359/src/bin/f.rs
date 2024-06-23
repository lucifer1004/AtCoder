use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        mut a: [i128; n],
    }

    let nn = n as i128;
    a.sort();
    let mut dec = BinaryHeap::new();
    dec.push((a[0] * (nn * 2 - 3), nn * 2 - 3, 0));
    let mut ans = a[0] * (nn - 1) * (nn - 1);

    let mut inc = BinaryHeap::new();
    for i in 1..n {
        inc.push((-a[i] * 3, 3, i));
        ans += a[i];
    }

    while dec.peek().unwrap().0 > -inc.peek().unwrap().0 {
        let (di, dcnt, did) = dec.pop().unwrap();
        let (ii, icnt, iid) = inc.pop().unwrap();
        ans -= di;
        ans -= ii;

        if dcnt > 2 {
            dec.push((a[did] * (dcnt - 2), dcnt - 2, did));
        }
        inc.push((-a[iid] * (icnt + 2), icnt + 2, iid));
    }

    println!("{}", ans);
}
