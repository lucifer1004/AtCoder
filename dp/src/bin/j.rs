use proconio::input;

fn solve(one: usize, two: usize, three: usize, n: usize, memo: &mut Vec<Vec<Vec<f64>>>) -> f64 {
    if memo[one][two][three] >= 0. {
        memo[one][two][three]
    } else {
        let mut e = 0.;
        let tot = (one + two + three) as f64;
        if three > 0 {
            e += solve(one, two + 1, three - 1, n, memo) * three as f64 / tot;
        }
        if two > 0 {
            e += solve(one + 1, two - 1, three, n, memo) * two as f64 / tot;
        }
        if one > 0 {
            e += solve(one - 1, two, three, n, memo) * one as f64 / tot;
        }
        e += n as f64 / tot;
        memo[one][two][three] = e;
        e
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = [0usize; 4];
    for ai in a {
        cnt[ai] += 1;
    }

    let mut memo: Vec<Vec<Vec<f64>>> = vec![vec![vec![-1.; n + 1]; n + 1]; n + 1];
    memo[0][0][0] = 0.;
    println!("{}", solve(cnt[1], cnt[2], cnt[3], n, &mut memo));
}
