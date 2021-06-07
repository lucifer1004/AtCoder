use proconio::input;

const MOD: usize = 998_244_353;

fn mat_mul(a: Vec<Vec<usize>>, b: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = a.len();
    let k = a[0].len();
    assert_eq!(k, b.len());
    let m = b[0].len();
    let mut ans = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            for p in 0..k {
                ans[i][j] += a[i][p] * b[p][j] % MOD;
                ans[i][j] %= MOD;
            }
        }
    }
    ans
}

fn mat_exp(mut x: Vec<Vec<usize>>, mut y: usize) -> Vec<Vec<usize>> {
    let n = x.len();
    assert_eq!(n, x[0].len());
    let mut ans = vec![vec![0; n]; n];
    for i in 0..n {
        ans[i][i] = 1;
    }
    while y > 0 {
        if (y & 1) > 0 {
            ans = mat_mul(ans.clone(), x.clone());
        }
        x = mat_mul(x.clone(), x.clone());
        y >>= 1;
    }
    ans
}

fn dfs(h: usize, pre: usize, acc: usize, pos: usize, trans: &mut Vec<Vec<usize>>) {
    if pos == h {
        trans[acc][pre] += 1;
        return;
    }
    if pre & (1 << pos) == 0 {
        dfs(h, pre, acc ^ (1 << pos), pos + 1, trans);
    } else {
        dfs(h, pre, acc, pos + 1, trans);
        dfs(h, pre, acc ^ (1 << pos), pos + 1, trans);
        if pos + 2 <= h && (pre & (1 << (pos + 1)) > 0) {
            dfs(h, pre, acc ^ (3 << pos), pos + 2, trans);
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut trans = vec![vec![0; 1 << h]; 1 << h];
    for pre in 0..(1 << h) {
        dfs(h, pre, 0, 0, &mut trans);
    }

    let t = mat_exp(trans, w);
    let mut start = vec![vec![0]; 1 << h];
    start[(1 << h) - 1][0] = 1;
    let end = mat_mul(t, start);
    println!("{}", end[(1 << h) - 1][0]);
}
