use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let sb = b.clone().into_iter().collect::<HashSet<_>>();
    let mut same = 0;
    let mut diff = 0;
    for i in 0..n {
        if sb.contains(&a[i]) {
            if b[i] == a[i] {
                same += 1;
            } else {
                diff += 1;
            }
        }
    }

    println!("{}\n{}", same, diff);
}
