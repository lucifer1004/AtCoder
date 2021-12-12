use std::collections::HashSet;
use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

const N: usize = 400;
const M: usize = 1995;

#[derive(Debug, Clone)]
struct DisjointSetUnion {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSetUnion {
    pub fn new(n: usize) -> Self {
        DisjointSetUnion {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    pub fn merge(&mut self, u: usize, v: usize) {
        let ru = self.root(u);
        let rv = self.root(v);
        if ru == rv {
            return;
        }
        if self.size[ru] >= self.size[rv] {
            self.parent[rv] = ru;
            self.size[ru] += self.size[rv];
        } else {
            self.parent[ru] = rv;
            self.size[rv] += self.size[ru];
        }
    }

    pub fn is_connected(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    fn root(&mut self, u: usize) -> usize {
        if self.parent[u] == u {
            u
        } else {
            self.parent[u] = self.root(self.parent[u]);
            self.parent[u]
        }
    }
}

fn mst(edges: &Vec<(usize, usize, f64)>, chosen: &HashSet<usize>, start: usize) -> f64 {
    let mut dsu = DisjointSetUnion::new(N);
    let mut cost = 0.0;
    let mut rem = N - 1;

    for &i in chosen.iter() {
        dsu.merge(edges[i].0, edges[i].1);
        cost += edges[i].2;
        rem -= 1;
    }

    let mut sorted_edges = edges.clone().iter().enumerate()
        .filter(|(i, _)| *i > start)
        .map(|(_, &x)| x)
        .collect::<Vec<_>>();

    sorted_edges.sort_by(|&x, &y| {
        ((x.2).round() as usize).cmp(&((y.2).round() as usize))
    });

    for &(u, v, d) in sorted_edges.iter() {
        if !dsu.is_connected(u, v) {
            dsu.merge(u, v);
            cost += d;
            rem -= 1;
        }
    }

    if rem == 0 { cost } else { 1e18 }
}

fn read<T: FromStr>() -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line
        .trim()
        .split(" ")
        .filter(|&s| s.len() > 0)
        .map(|s| s.parse::<T>().unwrap())
        .collect();
}

fn main() {
    let mut nodes = vec![];
    for _ in 0..N {
        let pos = read::<f64>();
        nodes.push((pos[0], pos[1]));
    }

    let mut edges = vec![];
    for _ in 0..M {
        let edge = read::<usize>();
        edges.push((edge[0], edge[1]));
    }

    let mut adj = vec![vec![]; N];
    let mut e = vec![];
    for (i, &(u, v)) in edges.iter().enumerate() {
        let d = 2.0 * ((nodes[u].0 - nodes[v].0) * (nodes[u].0 - nodes[v].0) + (nodes[u].1 - nodes[v].1) * (nodes[u].1 - nodes[v].1)).sqrt();
        adj[u].push(i);
        adj[v].push(i);
        e.push((u, v, d));
    }

    let mut dsu = DisjointSetUnion::new(N);
    let mut chosen = HashSet::new();

    for i in 0..M {
        e[i] = (e[i].0, e[i].1, read::<f64>()[0]);

        if dsu.is_connected(e[i].0, e[i].1) {
            println!("0");
            std::io::stdout().flush().unwrap();
            continue;
        }

        let mut chosen_bak = chosen.clone();
        chosen_bak.insert(i);

        let c1 = mst(&e, &chosen_bak, i);
        let c2 = mst(&e, &chosen, i);

        if c2 < 0.0 || c1 < c2 {
            println!("1");
            chosen.insert(i);
            dsu.merge(e[i].0, e[i].1);
        } else {
            println!("0");
        }

        std::io::stdout().flush().unwrap();
    }
}
