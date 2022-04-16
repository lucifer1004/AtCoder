use proconio::input;

fn main() {
    input! {
        n: usize,
        target: usize,
        pts: [(i64, i64); n],
    }

    if target == 1 {
        println!("Infinity");
        return;
    }

    let mut vis = vec![vec![false; n]; n];
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if vis[i][j] {
                continue;
            }

            let mut v = vec![i, j];
            for k in j + 1..n {
                if (pts[k].1 - pts[i].1) * (pts[j].0 - pts[i].0)
                    == (pts[k].0 - pts[i].0) * (pts[j].1 - pts[i].1)
                {
                    v.push(k);
                }
            }

            if v.len() >= target {
                ans += 1;
            }

            for &p in v.iter() {
                for &q in v.iter() {
                    vis[p][q] = true;
                }
            }
        }
    }

    println!("{}", ans);
}
