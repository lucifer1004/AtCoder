use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        points: [(i64, i64); n],
    }

    let mut buckets: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    let mut ans: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        let x = points[i].0 / k;
        let y = points[i].1 / k;
        for xp in x - 1..=x + 1 {
            for yp in y - 1..=y + 1 {
                if !buckets.contains_key(&(xp, yp)) {
                    continue;
                }

                for &j in buckets.get(&(xp, yp)).unwrap() {
                    if (points[i].0 - points[j].0) * (points[i].0 - points[j].0) + (points[i].1 - points[j].1) * (points[i].1 - points[j].1) <= k * k {
                        ans.push((j, i));
                    }
                }
            }
        }
        (*buckets.entry((x, y)).or_default()).push(i);
    }

    println!("{}", ans.len());
    ans.sort();
    for &(i, j) in ans.iter() {
        println!("{} {}", i + 1, j + 1);
    }
}
