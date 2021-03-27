use proconio::input;

const INF: i64 = 1_000_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        balls: [(i64, usize); n],
    }

    let mut pos = vec![vec![]; n + 2];
    let mut left = vec![INF; n + 2];
    let mut right = vec![-INF; n + 2];

    left[0] = 0;
    right[0] = 0;
    left[n + 1] = 0;
    right[n + 1] = 0;

    for (x, c) in balls {
        left[c] = left[c].min(x);
        right[c] = right[c].max(x);
    }

    let rl = |x: i64, i: usize| {
        if x <= right[i] {
            right[i] * 2 - x - left[i]
        } else {
            x - left[i]
        }
    };

    let lr = |x: i64, i: usize| {
        if x >= left[i] {
            x + right[i] - left[i] * 2
        } else {
            right[i] - x
        }
    };

    let mut last = 0;
    let mut lc = 0;
    let mut rc = 0;
    for i in 1..=n + 1 {
        if left[i] != INF {
            let nlc = (lc + rl(left[last], i)).min(rc + rl(right[last], i));
            let nrc = (lc + lr(left[last], i)).min(rc + lr(right[last], i));

            last = i;
            lc = nlc;
            rc = nrc;
        }
    }

    println!("{}", lc);
}
