use proconio::input;

const N: usize = 26;

fn count(s: &Vec<char>) -> [usize; N] {
    let mut ans = [0; N];
    for i in 0..s.len() {
        ans[(s[i] as u8 - 'a' as u8) as usize] += 1;
    }
    ans
}

fn merge(a: &[usize; N], b: &[usize; N]) -> [usize; N] {
    let mut ans = [0usize; N];
    for i in 0..N {
        ans[i] = a[i] + b[i];
    }
    ans
}

fn calculate(precalculated: &Vec<(usize, [usize; N])>, s: &Vec<char>, n: usize, len: usize) -> [usize; N] {
    if len <= n {
        let mut ans = [0usize; N];
        for i in 0..len {
            ans[(s[i] as u8 - 'a' as u8) as usize] += 1;
        }
        ans
    } else {
        let mut idx = 1;
        while precalculated[idx - 1].0 + precalculated[idx].0 <= len {
            idx += 1;
        }
        let mut ans = precalculated[idx].1.clone();
        let mut rem = calculate(precalculated, s, n, len - precalculated[idx].0);
        for i in 0..N {
            ans[i] += rem[i];
        }
        ans
    }
}

fn main() {
    input! {
        ss: String,
        mut l: usize,
        mut r: usize,
    }

    let n = ss.len() / 2;
    let mut s = vec!['a'; n];
    s.clone_from_slice(&ss.chars().collect::<Vec<char>>()[0..n]);
    let mut pi = vec![0; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            pi[i] = j + 1;
        }
    }

    let k = n - pi[n - 1];
    let mut t = vec!['a'; k];
    t.clone_from_slice(&s[0..k]);
    let mut precalculated: Vec<(usize, [usize; N])> = vec![];
    precalculated.push((k, count(&t)));
    precalculated.push((n, count(&s)));
    let mut idx = 1;
    while precalculated[idx - 1].0 + precalculated[idx].0 <= r {
        precalculated.push((precalculated[idx - 1].0 + precalculated[idx].0, merge(&precalculated[idx - 1].1, &precalculated[idx].1)));
        idx += 1;
    }

    let lcnt = calculate(&precalculated, &s, n, l - 1);
    let rcnt = calculate(&precalculated, &s, n, r);

    println!(
        "{}",
        (0..26)
            .into_iter()
            .map(|x| (rcnt[x] - lcnt[x]).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
