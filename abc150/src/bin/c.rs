use proconio::input;

fn calc(v: Vec<usize>, n: usize) -> usize {
    let mut ans = 0;
    let mut rem = 1;
    for i in 1..n {
        rem *= i;
    }
    let mut used = vec![false; n + 1];

    for i in 0..n {
        let mut order = 0;
        for j in 1..v[i] {
            if !used[j] {
                order += 1;
            }
        }
        ans += order * rem;
        if i != n - 1 {
            rem /= n - 1 - i;
        }
        used[v[i]] = true;
    }

    ans
}

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let cp = calc(p, n);
    let cq = calc(q, n);

    println!("{}", (cp.max(cq)) - (cp.min(cq)));
}
