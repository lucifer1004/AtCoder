use proconio::input;
use std::collections::HashMap;

const INF: i64 = 1_000_000_000_000_000_000;

fn main() {
    input! {
        n: i64,
        mut x: i64,
        mut d: i64,
    };

    if d == 0 {
        if x == 0 {
            println!("1");
        } else {
            println!("{}", n + 1);
        }
    } else {
        if d < 0 {
            d = -d;
            x = -x;
        }

        let mut mp: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
        for k in 0..=n {
            let key = ((x * k) % d + d) % d;
            let entry = mp.entry(key).or_insert(vec![]);
            let left = x * k + k * (k - 1) / 2 * d;
            let right = x * k + (2 * n - k - 1) * k / 2 * d;
            entry.push((left, right));
        }

        let mut ans = 0i64;
        for (_key, mut val) in mp {
            val.sort();
            let mut left = -INF;
            let mut right = -INF;
            for (l, r) in val {
                if l > right {
                    if right != -INF {
                        ans += (right - left) / d + 1;
                    }
                    left = l;
                    right = r;
                } else {
                    right = right.max(r);
                }
            }
            if right != -INF {
                ans += (right - left) / d + 1;
            }
        }

        println!("{}", ans);
    }
}
