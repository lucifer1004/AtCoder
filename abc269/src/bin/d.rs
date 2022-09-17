use proconio::input;
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        points: [(i32, i32); n],
    }

    let d = [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];
    let s = points.into_iter().collect::<HashSet<_>>();
    let mut vis: HashSet<(i32, i32)> = HashSet::new();
    let mut ans = 0;
    for &(i, j) in s.iter() {
        if vis.contains(&(i, j)) {
            continue;
        }
        ans += 1;
        let mut dq: VecDeque<(i32, i32)> = VecDeque::new();
        dq.push_back((i, j));
        vis.insert((i, j));
        while let Some((i, j)) = dq.pop_front() {
            for &(di, dj) in d.iter() {
                if s.contains(&(i + di, j + dj)) && !vis.contains(&(i + di, j + dj)) {
                    vis.insert((i + di, j + dj));
                    dq.push_back((i + di, j + dj));
                }
            }
        }
    }
    println!("{}", ans);
}
