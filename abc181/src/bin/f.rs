use proconio::input;

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
        points: [(f64, f64); n],
    }

    let mut edges: Vec<(f64, usize, usize)> = vec![];
    let s = 0usize;
    let t = n + 1;
    for (i, (x, y)) in points.clone().into_iter().enumerate() {
        let idx = i + 1;
        edges.push((100. - y, s, idx));
        edges.push((y + 100., idx, t));
        for j in i + 1..n {
            let (x1, y1) = points[j];
            let dis = ((x1 - x) * (x1 - x) + (y1 - y) * (y1 - y)).sqrt();
            edges.push((dis, idx, j + 1));
        }
    }
    edges.push((200., s, t));

    edges.sort_by(|a, b| {
        if a.0 < b.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut dsu = DisjointSetUnion::new(n + 2);

    for (d, u, v) in edges {
        dsu.merge(u, v);
        if dsu.is_connected(s, t) {
            println!("{}", d / 2.);
            return;
        }
    }
}
