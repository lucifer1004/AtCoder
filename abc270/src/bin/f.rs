use proconio::input;
use proconio::marker::Usize1;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        Self {
            parent,
            rank,
            components: n,
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.parent[x];
            let r = self.root(p);
            self.parent[x] = r;
            r
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
        self.components -= 1;
    }
}

struct MinimumSpanningTree {
    n: usize,
    edges: Vec<(usize, usize, usize)>,
}

impl MinimumSpanningTree {
    fn new(n: usize, edges: Vec<(usize, usize, usize)>) -> Self {
        Self { n, edges }
    }

    fn solve(&self) -> usize {
        let mut edges = self.edges.clone();
        edges.sort_by_key(|x| x.2);

        let mut uf = UnionFind::new(self.n);
        let mut cost = 0;
        for (u, v, w) in edges {
            if uf.same(u, v) {
                continue;
            }
            uf.unite(u, v);
            cost += w;
        }

        if uf.components == 1 {
            cost
        } else {
            std::usize::MAX
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n], // cost of building airport
        y: [usize; n], // cost of building harbor
        edges: [(Usize1, Usize1, usize); m], // cost of building road
    }

    // This is a minimum spanning tree problem.
    let mut ans = std::usize::MAX;

    // First time, only build roads
    let only_roads = MinimumSpanningTree::new(n, edges.clone());
    ans = ans.min(only_roads.solve());

    // Second time, build roads and airports
    let mut edges_with_airports = edges.clone();
    for i in 0..n {
        edges_with_airports.push((i, n, x[i]));
    }
    let roads_and_airports = MinimumSpanningTree::new(n + 1, edges_with_airports);
    ans = ans.min(roads_and_airports.solve());

    // Third time, build roads and harbors
    let mut edges_with_harbors = edges.clone();
    for i in 0..n {
        edges_with_harbors.push((i, n, y[i]));
    }
    let roads_and_harbors = MinimumSpanningTree::new(n + 1, edges_with_harbors);
    ans = ans.min(roads_and_harbors.solve());

    // Last time, build roads, airports, and harbors
    let mut edges_with_airports_and_harbors = edges.clone();
    for i in 0..n {
        edges_with_airports_and_harbors.push((i, n, x[i]));
        edges_with_airports_and_harbors.push((i, n + 1, y[i]));
    }
    let roads_airports_and_harbors =
        MinimumSpanningTree::new(n + 2, edges_with_airports_and_harbors);
    ans = ans.min(roads_airports_and_harbors.solve());

    println!("{}", ans);
}
