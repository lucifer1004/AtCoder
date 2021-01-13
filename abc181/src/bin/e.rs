use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [i32; n],
        mut w: [i32; m],
    }

    let mut ans = std::i32::MAX;
    if n == 1 {
        for wi in w.clone() {
            ans = min(ans, (wi - h[0]).abs());
        }
        println!("{}", ans);
        return;
    }

    h.sort();
    w.sort();

    let mut left = vec![0; n];
    for i in (1..n).step_by(2) {
        if i >= 2 {
            left[i] += left[i - 2];
        }
        left[i] += h[i] - h[i - 1];
    }

    let mut right = vec![0; n];
    for i in (1..=n - 2).rev().step_by(2) {
        if i < n - 2 {
            right[i] += right[i + 2];
        }
        right[i] += h[i + 1] - h[i];
    }

    let mut ptr = 0usize;
    for wi in w {
        while ptr < n && wi > h[ptr] {
            ptr += 1;
        }
        ans = min(
            ans,
            if ptr % 2 == 0 {
                h[ptr] - wi
                    + (if ptr + 1 < n { right[ptr + 1] } else { 0 })
                    + (if ptr >= 2 { left[ptr - 1] } else { 0 })
            } else {
                wi - h[ptr - 1]
                    + (if ptr < n { right[ptr] } else { 0 })
                    + (if ptr >= 3 { left[ptr - 2] } else { 0 })
            },
        )
    }

    println!("{}", ans);
}
