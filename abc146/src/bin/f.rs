use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars
    }

    let mut dq: VecDeque<usize> = VecDeque::new();
    let mut pre = vec![n + 1; n + 1];

    dq.push_back(0usize);

    for i in 1..=n {
        while !dq.is_empty() && *dq.front().unwrap() + m < i {
            dq.pop_front();
        }

        if s[i] == '0' && !dq.is_empty() {
            pre[i] = *dq.front().unwrap();
            dq.push_back(i);
        }
    }

    if pre[n] == n + 1 {
        println!("-1");
    } else {
        let mut seq = vec![];
        let mut curr = n;
        while curr != n + 1 {
            seq.push(curr);
            curr = pre[curr];
        }
        seq.reverse();
        for i in 0..seq.len() - 1 {
            print!("{} ", seq[i + 1] - seq[i]);
        }
    }
}
