use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        k: usize,
        a: [[i64]; k],
    };

    let ops = a.clone().into_iter().fold(0, |acc, x| acc + x.len()) - k;
    let f = |v: &Vec<i64>, l: usize, r: usize| -> i64 {
        assert!(l <= r);
        assert_eq!((r - l + 1) % 2, 1);

        if l == r {
            v[l]
        } else {
            let mid = (l + r) >> 1;
            if ops % 2 == 0 {
                v[mid].min(v[mid - 1].max(v[mid + 1]))
            } else {
                v[mid].max(v[mid - 1].min(v[mid + 1]))
            }
        }
    };

    let mut left = vec![0usize; k];
    let mut right = a
        .clone()
        .into_iter()
        .map(|x| x.len() - 1)
        .collect::<Vec<usize>>();

    let mut ans = 0;
    let mut pq: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    let mut delta = vec![0i64; k];
    let mut odds = vec![];
    let mut finished = 0;
    for i in 0..k {
        if left[i] == right[i] {
            ans += a[i][0];
            finished += 1;
        } else if a[i].len() % 2 == 1 {
            odds.push(i);
        } else {
            let take_first = f(&a[i], left[i] + 1, right[i]);
            let take_last = f(&a[i], left[i], right[i] - 1);
            delta[i] = take_first.max(take_last) - take_first.min(take_last);
            pq.push((delta[i], i));
        }
    }

    let mut takahashi_move = true;
    while finished < k {
        while !pq.is_empty() {
            let (_, i) = pq.pop().unwrap();
            let take_first = f(&a[i], left[i] + 1, right[i]);
            let take_last = f(&a[i], left[i], right[i] - 1);
            if (take_first <= take_last && takahashi_move) || (take_first >= take_last && !takahashi_move) {
                right[i] -= 1;
            } else {
                left[i] += 1;
            }
            if left[i] == right[i] {
                ans += a[i][left[i]];
                finished += 1;
            } else {
                odds.push(i);
            }

            takahashi_move = !takahashi_move;
        }

        if !odds.is_empty() {
            let i = odds.pop().unwrap();
            let mid = (left[i] + right[i]) >> 1;
            if (a[i][mid - 1] <= a[i][mid + 1] && takahashi_move) || (a[i][mid - 1] >= a[i][mid + 1] && !takahashi_move) {
                left[i] += 1;
            } else {
                right[i] -= 1;
            }
            let take_first = f(&a[i], left[i] + 1, right[i]);
            let take_last = f(&a[i], left[i], right[i] - 1);
            delta[i] = take_first.max(take_last) - take_first.min(take_last);
            pq.push((delta[i], i));

            takahashi_move = !takahashi_move;
        }
    }

    println!("{}", ans);
}
