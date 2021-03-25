use std::collections::HashSet;

use proconio::input;

fn z_function(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0usize; n];
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i <= r {
            z[i] = (r - i + 1).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

fn main() {
    input! {
        s: String,
    }

    let n = s.len();
    let t = [s.clone(), s.clone()].concat();
    if t[1..n * 2 - 1].find(&s).is_none() {
        println!("1\n1");
        std::process::exit(0);
    }
    if s.chars().collect::<HashSet<char>>().len() == 1 {
        println!("{}\n1", s.len());
        std::process::exit(0);
    }

    let mut s = s.chars().collect::<Vec<char>>();
    let pre = z_function(&s);

    s.reverse();
    let suf = z_function(&s);

    let mut valid = vec![true; n];
    for i in 1..n - 1 {
        for j in (i..=pre[i]).step_by(i) {
            valid[i + j - 1] = false;
        }
        for j in (i..=suf[i]).step_by(i) {
            if n - (i + j) > 0 {
                valid[n - (i + j) - 1] = false;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        if valid[i] {
            ans += 1;
        }
    }
    println!("2\n{}", ans);
}
