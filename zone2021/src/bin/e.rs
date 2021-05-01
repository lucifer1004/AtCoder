use std::collections::BinaryHeap;

use proconio::input;

const INF: i64 = 1_000_000_000;

fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[i64; c - 1]; r],
        b: [[i64; c]; r - 1],
    }

    let mut pq: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    let mut dist = vec![vec![INF; c]; r];
    dist[0][0] = 0;
    pq.push((0, 0, 0));

    while !pq.is_empty() {
        let (cost_neg, row, col) = pq.pop().unwrap();
        let cost = -cost_neg;
        if cost > dist[row][col] {
            continue;
        }
        if row == r - 1 && col == c - 1 {
            println!("{}", cost);
            std::process::exit(0);
        }

        if col < c - 1 && cost + a[row][col] < dist[row][col + 1] {
            dist[row][col + 1] = cost + a[row][col];
            pq.push((-dist[row][col + 1], row, col + 1));
        }

        if col >= 1 && cost + a[row][col - 1] < dist[row][col - 1] {
            dist[row][col - 1] = cost + a[row][col - 1];
            pq.push((-dist[row][col - 1], row, col - 1));
        }

        if row < r - 1 && cost + b[row][col] < dist[row + 1][col] {
            dist[row + 1][col] = cost + b[row][col];
            pq.push((-dist[row + 1][col], row + 1, col));
        }

        for i in 1..=row {
            let nc = cost + 1 + i as i64;
            if nc > dist[row - i][col] {
                break;
            }
            if nc < dist[row - i][col] {
                dist[row - i][col] = nc;
                pq.push((-nc, row - i, col));
            }
        }
    }
}