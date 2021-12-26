use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut counter: HashMap<i64, usize> = HashMap::new();
    counter.insert(0, 1);

    let mut sum: i64 = 0;
    let mut ans: usize = 0;
    for &num in a.iter() {
        sum += num;
        if counter.contains_key(&(sum - k)) {
            ans += counter.get(&(sum - k)).unwrap();
        }
        *counter.entry(sum).or_insert(0) += 1;
    }

    println!("{}", ans);
}
