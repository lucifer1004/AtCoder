use proconio::input;

const MOD: usize = 1_000_000_007;

fn identity(n: usize) -> Vec<Vec<usize>> {
    let mut ans = vec![vec![0usize; n]; n];
    for i in 0..n {
        ans[i][i] = 1;
    }

    ans
}

fn mat_mul(a: Vec<Vec<usize>>, b: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = a.len();
    assert!(n > 0);
    let m = a[0].len();
    let m2 = b.len();
    assert!(m == m2 && m > 0);
    let k = b[0].len();
    let mut ans = vec![vec![0usize; k]; n];
    for i in 0..n {
        for j in 0..k {
            for p in 0..m {
                ans[i][j] = (ans[i][j] + a[i][p] * b[p][j]) % MOD;
            }
        }
    }
    ans
}

fn mat_exp(mut a: Vec<Vec<usize>>, mut y: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    assert!(n > 0 && a[0].len() == n);

    let mut ans = identity(n);

    while y > 0 {
        if y & 1 == 1 {
            ans = mat_mul(ans.clone(), a.clone());
        }
        a = mat_mul(a.clone(), a.clone());
        y = y >> 1;
    }

    ans
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    }

    let mut transition = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                transition[j][i] = 1;
            }
        }
    }

    transition = mat_exp(transition.clone(), k);
    let mut result = mat_mul(transition, vec![vec![1usize]; n]);
    let mut ans = 0;
    for i in 0..n {
        ans = (ans + result[i][0]) % MOD;
    }
    println!("{}", ans);
}