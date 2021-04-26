use proconio::input;
use proconio::marker::Usize1;

const MOD: usize = 1_000_000_007;

fn mod_exp(mut x: usize, mut y: usize) -> usize {
    let mut ans = 1;
    while y > 0 {
        if y & 1 == 1 {
            ans = ans * x % MOD;
        }
        x = x * x % MOD;
        y >>= 1;
    }
    ans
}

fn mod_inv(x: usize) -> usize {
    mod_exp(x, MOD - 2)
}

fn mat_mul(x: Vec<Vec<usize>>, y: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = x.len();
    assert!(n > 0);
    let m = x[0].len();
    assert!(m > 0 && y.len() == m);
    let k = y[0].len();
    let mut ans = vec![vec![0; k]; n];
    for i in 0..n {
        for j in 0..k {
            for p in 0..m {
                ans[i][j] = (ans[i][j] + x[i][p] * y[p][j]) % MOD;
            }
        }
    }

    ans
}

fn mat_exp(mut x: Vec<Vec<usize>>, mut y: usize) -> Vec<Vec<usize>> {
    let n = x.len();
    assert!(n > 0 && x[0].len() == n);

    let mut ans = vec![vec![0; n]; n];
    for i in 0..n {
        ans[i][i] = 1;
    }

    while y > 0 {
        if y & 1 == 1 {
            ans = mat_mul(ans.clone(), x.clone());
        }
        x = mat_mul(x.clone(), x.clone());
        y >>= 1;
    }

    ans
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        edges: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let start = a.iter().map(|x| vec![*x]).collect::<Vec<Vec<usize>>>();

    let half = mod_inv(2);
    let im = mod_inv(m);
    let half_im = half * im % MOD;
    let mut transit = vec![vec![0; n]; n];
    for i in 0..n {
        for j in adj[i].clone() {
            transit[i][j] = half_im;
        }
        transit[i][i] = (m * 2 - adj[i].len()) * half_im % MOD;
    }
    transit = mat_exp(transit, k);

    let ans = mat_mul(transit, start);

    for i in 0..n {
        println!("{}", ans[i][0]);
    }
}