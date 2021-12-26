use proconio::input;
use proconio::marker::Usize1;
use std::collections::{HashMap, HashSet, VecDeque};

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


fn main() {
    input! {
        n: usize,
        mut p: [Usize1; n],
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    let mut dsu = DisjointSetUnion::new(n);

    for (i, &(u, v)) in edges.iter().enumerate() {
        if dsu.is_connected(u, v) {
            continue;
        }
        dsu.merge(u, v);
        adj[u].push(i);
        adj[v].push(i);
    }

    let mut groups: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for i in 0..n {
        groups[dsu.root(i)].push((p[i], i));
    }

    for i in 0..n {
        for &u in groups[i].iter() {
            if dsu.root(u.0) != dsu.root(u.1) {
                println!("-1");
                std::process::exit(0);
            }
        }
    }

    let mut ans: Vec<usize> = vec![];
    let mut cpos: Vec<usize> = vec![0; n];
    let mut fixed: Vec<bool> = vec![false; n];
    for i in 0..n {
        cpos[p[i]] = i;
    }

    for i in 0..n {
        if groups[i].len() <= 1 {
            continue;
        }

        let mut order: Vec<usize> = vec![];
        let mut vis: HashSet<usize> = HashSet::new();
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(groups[i][0].0);
        vis.insert(groups[i][0].0);
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            order.push(u);
            for &j in adj[u].iter() {
                let v = edges[j].0 + edges[j].1 - u;
                if !vis.contains(&v) {
                    q.push_back(v);
                    vis.insert(v);
                }
            }
        }
        order.reverse();

        for &u in order.iter() {
            let c = cpos[u];
            if c != u {
                let mut qq: VecDeque<usize> = VecDeque::new();
                let mut pre: HashMap<usize, usize> = HashMap::new();
                pre.insert(u, m);
                qq.push_back(u);
                while !qq.is_empty() {
                    let uu = qq.pop_front().unwrap();
                    for &j in adj[uu].iter() {
                        let vv = edges[j].0 + edges[j].1 - uu;
                        if fixed[vv] || pre.contains_key(&vv) {
                            continue;
                        }
                        pre.insert(vv, j);
                        qq.push_back(vv);
                    }
                }

                assert!(pre.contains_key(&u));

                let mut ptr = c;
                while ptr != u {
                    let j = *pre.get(&ptr).unwrap();
                    ans.push(j);
                    let nxt = edges[j].0 + edges[j].1 - ptr;
                    let tmp = p[ptr];
                    p[ptr] = p[nxt];
                    p[nxt] = tmp;
                    cpos[p[ptr]] = ptr;
                    cpos[p[nxt]] = nxt;
                    ptr = nxt;
                }
            }

            assert!(p[u] == u && cpos[u] == u);

            fixed[u] = true;
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.into_iter().map(|x| (x + 1).to_string()).collect::<Vec<String>>().join(" "));
}
