use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        positions: [(i64, i64); n],
        s: String,
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut rows: HashMap<i64, Vec<(i64, usize)>> = HashMap::new();
    for (i, &(x, y)) in positions.iter().enumerate() {
        rows.entry(y).or_insert(vec![]).push((x, i));
    }

    for row in rows.values() {
        let mut row = row.clone();
        row.sort_unstable();
        let m = row.len();
        for i in 0..m - 1 {
            let (_, j1) = row[i];
            let (_, j2) = row[i + 1];
            if s[j1] == 'R' && s[j2] == 'L' {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
