use proconio::input;

const K: usize = 30;
const INF: usize = 1 << K;

fn get_bit(v: Vec<usize>, bit: usize) -> Vec<usize> {
    let mut ans = vec![0; v.len()];
    let mask = 1 << bit;
    for i in 0..v.len() {
        if v[i] & mask != 0 {
            ans[i] = 1;
        }
    }
    ans
}

fn flip(v: Vec<usize>) -> Vec<usize> {
    v.into_iter().map(|x| 1 - x).collect::<Vec<usize>>()
}

fn kmp(pattern: Vec<usize>, check: Vec<usize>) -> Vec<usize> {
    let mut pos = vec![];

    let n = pattern.len();
    let s = [pattern, [INF].to_vec(), check.clone(), check].concat();
    let mut pi = vec![0; s.len()];
    for i in 1..s.len() - 1 {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            pi[i] = j + 1;
        }
        if pi[i] == n {
            pos.push(i - n * 2);
        }
    }

    pos
}

/// TODO: We only need to run KMP once.
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut pos = vec![0; n];

    for i in 0..K {
        let mut good = vec![0; n];
        let sa = get_bit(a.clone(), i);
        let fa = flip(sa.clone());
        let sb = get_bit(b.clone(), i);

        for p in kmp(sb.clone(), sa) {
            good[p] = 1;
        }
        for p in kmp(sb.clone(), fa) {
            good[p] = 2;
        }

        for p in 0..n {
            match good[p] {
                0 => pos[p] = INF,
                2 => pos[p] ^= 1 << i,
                _ => {}
            }
        }
    }

    for i in 0..n {
        if pos[i] != INF {
            println!("{} {}", i, pos[i]);
        }
    }
}
