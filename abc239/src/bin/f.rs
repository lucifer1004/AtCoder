use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: [usize; n],
        edges: [(Usize1, Usize1); m],
    }

    let mut dsum = 0;
    for &di in d.iter() {
        dsum += di;
    }
    if dsum != 2 * (n - 1) {
        println!("-1");
        return;
    }

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut rem: Vec<usize> = vec![0; n];
    for i in 0..n {
        if adj[i].len() > d[i] {
            println!("-1");
            return;
        }
        rem[i] = d[i] - adj[i].len();
    }

    let mut vis = vec![false; n];
    let mut groups = vec![];
    for i in 0..n {
        if !vis[i] {
            vis[i] = true;
            let mut group = vec![i];
            let mut stk = vec![(i, n)];
            while !stk.is_empty() {
                let (u, p) = stk.pop().unwrap();
                for &v in adj[u].iter() {
                    if v == p {
                        continue;
                    }
                    if vis[v] {
                        println!("-1");
                        return;
                    }
                    vis[v] = true;
                    group.push(v);
                    stk.push((v, u));
                }
            }
            groups.push(group);
        }
    }

    let mut rtot = vec![vec![]; groups.len()];
    let mut sorted_groups = vec![];
    for i in 0..groups.len() {
        for &u in groups[i].iter() {
            for _ in 0..rem[u] {
                rtot[i].push(u);
            }
        }
        sorted_groups.push((rtot[i].len(), i));
    }
    sorted_groups.sort();
    sorted_groups.reverse();

    let mut ptr = 1;
    let mut ans: Vec<(usize, usize)> = vec![];
    for &(_, u) in sorted_groups.iter() {
        let start = if u == sorted_groups[0].1 { 0 } else { 1 };
        for i in start..rtot[u].len() {
            if ptr >= sorted_groups.len() || rtot[sorted_groups[ptr].1].len() == 0 {
                println!("-1");
                return;
            }
            ans.push((rtot[u][i], rtot[sorted_groups[ptr].1][0]));
            ptr += 1;
        }
    }

    for (u, v) in ans {
        println!("{} {}", u + 1, v + 1);
    }
}
