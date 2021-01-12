use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut lo = 1usize;
    let mut hi = n / 2;
    while lo <= hi {
        let mid = (lo + hi) >> 1;
        let mut mp = HashMap::new();
        for i in 0..n - mid + 1 {
            let key = s.clone()[i..i + mid].to_string();
            mp.entry(key).or_insert_with(Vec::new).push(i);
        }
        let mut found = false;
        for (_key, v) in mp.iter() {
            if v.len() >= 2 && v[v.len() - 1] - v[0] >= mid {
                found = true;
                break;
            }
        }
        if found {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    println!("{}", hi);
}
