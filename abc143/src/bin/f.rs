use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = vec![0usize; n + 1];
    for ai in a {
        cnt[ai] += 1;
    }

    let mut acc = vec![0usize; n + 1];
    for i in 1..=n {
        acc[cnt[i]] += 1;
    }
    for i in (1..n).rev() {
        acc[i] += acc[i + 1];
    }
    acc[0] = 0;
    for i in 1..=n {
        acc[i] += acc[i - 1];
    }

    let mut ans = vec![0usize; n + 1];
    let mut ptr = n;
    for i in 1..=n {
        while ptr > 0 && acc[ptr] / ptr < i {
            ptr -= 1;
        }
        ans[i] = ptr;
    }

    println!(
        "{}",
        ans[1..=n]
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
