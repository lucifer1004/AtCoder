use proconio::input;

const INF: i32 = 1_000_000_000;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
        b: [[i32; w]; h],
    }

    let mut last = vec![Vec::new(); w];
    last[0].push(0);

    for i in 0..h {
        let mut now = vec![Vec::new(); w];
        for j in 0..w {
            let delta = a[i][j] - b[i][j];
            let mut p1 = 0;
            let mut p2 = 0;
            let len1 = last[j].len();
            let mut p3 = 0;
            let mut p4 = 0;
            let len2 = if j > 0 { now[j - 1].len() } else { 0 };
            while p1 < len1 || p2 < len1 || p3 < len2 || p4 < len2 {
                let mut lo = INF;
                if p1 < len1 {
                    lo = lo.min(last[j][p1] + delta);
                }
                if p2 < len1 {
                    lo = lo.min(last[j][p2] - delta);
                }
                if p3 < len2 {
                    lo = lo.min(now[j - 1][p3] + delta);
                }
                if p4 < len2 {
                    lo = lo.min(now[j - 1][p4] - delta);
                }
                if p1 < len1 && lo == last[j][p1] + delta {
                    p1 += 1;
                }
                if p2 < len1 && lo == last[j][p2] - delta {
                    p2 += 1;
                }
                if p3 < len2 && lo == now[j - 1][p3] + delta {
                    p3 += 1;
                }
                if p4 < len2 && lo == now[j - 1][p4] - delta {
                    p4 += 1;
                }
                now[j].push(lo);
            }
        }
        last = now;
    }

    let mut ans = 1_000_000_000;
    for k in last[w - 1].clone() {
        ans = ans.min(k.abs());
    }

    println!("{}", ans);
}
