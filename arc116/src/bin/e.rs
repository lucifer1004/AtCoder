use proconio::input;
use proconio::marker::Usize1;

const INF: usize = 1_000_000_000;

struct Context {
    adj: Vec<Vec<usize>>,
    parent: Vec<usize>,
    used: Vec<usize>,
    dis: Vec<usize>,
    can_reach: Vec<bool>,
    threshold: usize,
}

impl Context {
    fn check(&mut self) -> usize {
        self.dfs(0);
        self.used[0]
    }

    fn dfs(&mut self, u: usize) {
        let mut child = false;
        let mut target = 0;
        let mut source = INF;
        for i in 0..self.adj[u].len() {
            let v = self.adj[u][i];
            if v != self.parent[u] {
                child = true;
                self.parent[v] = u;
                self.dfs(v);
                if self.can_reach[v] {
                    source = source.min(self.dis[v] + 1);
                } else {
                    target = target.max(self.dis[v] + 1);
                }
                self.used[u] += self.used[v];
            }
        }

        if child {
            if target >= self.threshold {
                self.can_reach[u] = true;
                self.used[u] += 1;
                self.dis[u] = 0;
            } else if source + target <= self.threshold {
                self.can_reach[u] = true;
                self.dis[u] = source;
            } else {
                self.dis[u] = target;
            }
        }

        if u == 0 && !self.can_reach[u] {
            self.used[u] += 1;
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut lo = 1;
    let mut hi = n - 1;
    while lo <= hi {
        let mid = (lo + hi) >> 1;
        let mut ctx = Context {
            adj: adj.clone(),
            parent: vec![n; n],
            used: vec![0; n],
            dis: vec![0; n],
            can_reach: vec![false; n],
            threshold: mid,
        };
        if ctx.check() <= k {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    println!("{}", lo);
}
