use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut dq: VecDeque<char> = VecDeque::new();
    let mut rev = false;
    for c in s.chars() {
        if c != 'R' {
            if rev {
                dq.push_front(c);
            } else {
                dq.push_back(c);
            }
        } else {
            rev = !rev;
        }
    }

    let mut ans = Vec::new();
    if !rev {
        while !dq.is_empty() {
            if !ans.is_empty() && *dq.front().unwrap() == ans[ans.len() - 1] {
                ans.pop();
                dq.pop_front();
            } else {
                ans.push(dq.pop_front().unwrap());
            }
        }
    } else {
        while !dq.is_empty() {
            if !ans.is_empty() && *dq.back().unwrap() == ans[ans.len() - 1] {
                ans.pop();
                dq.pop_back();
            } else {
                ans.push(dq.pop_back().unwrap());
            }
        }
    }

    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}