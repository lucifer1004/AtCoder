use proconio::input;
use std::collections::HashSet;

struct Graph {
    n: usize,
    inner_loop: Vec<usize>,
    adj: Vec<HashSet<usize>>,
    parent: Vec<usize>,
    color: Vec<usize>,
    depth: Vec<usize>,
}

impl Graph {
    fn check(&mut self) {
        let m = self.inner_loop.len();
        assert!(m > 0);
        for i in 0..m {
            let r = (i + 1) % m;
            for j in 0..m {
                if j != i && j != r && self.adj[self.inner_loop[i]].contains(&self.inner_loop[j]) {
                    let mut nxt: Vec<usize> = vec![];
                    let mut k = j;
                    while k != i {
                        nxt.push(self.inner_loop[k]);
                        k = (k + 1) % m;
                    }
                    nxt.push(self.inner_loop[i]);
                    self.inner_loop = nxt;
                    self.check();
                    return;
                }
            }
        }
    }

    fn dfs(&mut self, u: usize) {
        self.color[u] = 1;
        let mut lo = 0;
        for v in self.adj[u].clone() {
            if self.color[v] == 0 {
                self.parent[v] = u;
                self.depth[v] = self.depth[u] + 1;
                self.dfs(v);
            } else if self.color[v] == 1 {
                if self.depth[v] > self.depth[lo] {
                    lo = v;
                }
            }
        }
        if lo != 0 && self.inner_loop.is_empty() {
            let mut p = u;
            while p != self.parent[lo] {
                self.inner_loop.push(p);
                p = self.parent[p];
            }
            self.inner_loop.reverse();
            self.check();
        }
        self.color[u] = 2;
    }

    pub fn new(n: usize) -> Self {
        Graph {
            n,
            inner_loop: vec![],
            adj: vec![HashSet::new(); n + 1],
            parent: vec![0; n + 1],
            color: vec![0; n + 1],
            depth: vec![0; n + 1],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].insert(v);
    }

    pub fn find_loop(&mut self) {
        for i in 1usize..=self.n {
            if self.color[i] == 0 {
                self.depth[i] = 1;
                self.dfs(i);
                if !self.inner_loop.is_empty() {
                    break;
                }
            }
        }
        if self.inner_loop.is_empty() {
            println!("-1");
        } else {
            println!("{}", self.inner_loop.len());
            for u in self.inner_loop.clone() {
                println!("{}", u);
            }
        }
    }
}

fn main() {
    input! {
        (n, m): (usize, usize)
    }

    let mut g = Graph::new(n);

    for _ in 0..m {
        input! {
            (u, v): (usize, usize)
        }
        g.add_edge(u, v);
    }

    g.find_loop()
}
