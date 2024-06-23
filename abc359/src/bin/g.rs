use proconio::input;
use proconio::marker::Usize1;

struct SparseTable<T> {
    val: Vec<Vec<T>>,
    log2: Vec<usize>,
    reduce_fn: Box<dyn Fn(T, T) -> T>,
}

impl<T> SparseTable<T>
where
    T: Default + Copy,
{
    fn new(n: usize, a: Vec<T>, reduce_fn: Box<dyn Fn(T, T) -> T>) -> Self {
        let log2n = (n as f64).log2().ceil() as usize;
        let mut log2 = vec![0; n + 1];
        for i in 2..=n {
            log2[i] = log2[i >> 1] + 1;
        }

        let mut val = vec![vec![T::default(); log2n + 1]; n];
        for i in 0..n {
            val[i][0] = a[i];
        }
        for j in 1..=log2n {
            for i in 0..n {
                if i + (1 << j) <= n {
                    val[i][j] = reduce_fn(val[i][j - 1], val[i + (1 << (j - 1))][j - 1]);
                }
            }
        }
        Self {
            val,
            log2,
            reduce_fn,
        }
    }

    fn query(&self, l: usize, r: usize) -> T {
        let j = self.log2[r - l];
        (self.reduce_fn)(self.val[l][j], self.val[r - (1 << j)][j])
    }
}

struct LCATree {
    depth: Vec<usize>,
    first: Vec<usize>,
    st: SparseTable<(usize, usize)>,
}

impl LCATree {
    fn dfs(
        node: usize,
        h: usize,
        adj: &[Vec<usize>],
        first: &mut [usize],
        euler: &mut Vec<usize>,
        depth: &mut [usize],
    ) {
        first[node] = euler.len();
        euler.push(node);
        depth[node] = h;

        for &to in &adj[node] {
            if first[to] == 0 && to != 0 {
                LCATree::dfs(to, h + 1, adj, first, euler, depth);
                euler.push(node);
            }
        }
    }

    fn new(adj: &[Vec<usize>], root: usize) -> Self {
        let n = adj.len();
        let mut first = vec![0; n];
        let mut euler = Vec::with_capacity(n * 2);
        let mut depth = vec![0; n];
        LCATree::dfs(root, 0, adj, &mut first, &mut euler, &mut depth);

        let st = SparseTable::new(
            euler.len(),
            euler.iter().copied().map(|x| (depth[x], x)).collect(),
            Box::new(|x, y| if x < y { x } else { y }),
        );
        Self { depth, first, st }
    }

    fn lca(&self, u: usize, v: usize) -> usize {
        let left = self.first[u];
        let right = self.first[v];
        let (l, r) = if left > right {
            (right, left)
        } else {
            (left, right)
        };
        let (_, node) = self.st.query(l, r);
        node
    }

    fn dist(&self, u: usize, v: usize) -> usize {
        let w = self.lca(u, v);
        self.depth[u] + self.depth[v] - 2 * self.depth[w]
    }
}

fn dfs(
    u: usize,
    p: usize,
    adj: &[Vec<usize>],
    color: &[usize],
    current_color: usize,
    total_dist: &mut usize,
) -> (usize, usize) {
    let mut num = 0;
    let mut dist = 0;

    let mut nn = vec![];
    let mut dd = vec![];
    for &v in &adj[u] {
        if v == p {
            continue;
        }
        let (n, d) = dfs(v, u, adj, color, current_color, total_dist);
        num += n;
        dist += d + n;
        nn.push(n);
        dd.push(d);
    }

    for i in 0..nn.len() {
        *total_dist += (num - nn[i]) * (dd[i] + nn[i]);
    }

    if color[u] == current_color {
        *total_dist += dist;
        num += 1;
    }
    (num, dist)
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        color: [Usize1; n],
    }

    // There is a tree with n nodes, and each node has a color.
    // First, we build the adjacency list of the tree.
    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    // Next, we build an LCA (Lowest Common Ancestor) table.
    let lca = LCATree::new(&adj, 0);

    // We use different ways for color group with size threshold sqrt(n)
    let mut color_groups = vec![vec![]; n];
    for i in 0..n {
        color_groups[color[i]].push(i);
    }

    let mut ans = 0;
    let threshold = (n as f64).sqrt() as usize;
    for i in 0..n {
        if color_groups[i].len() >= threshold {
            // Use a DFS to calculate the distance between all pairs of nodes in the same color group.
            let mut total_dist = 0;
            dfs(0, n, &adj, &color, i, &mut total_dist);
            ans += total_dist;
        } else if color_groups[i].len() >= 2 {
            for &u in &color_groups[i] {
                for &v in &color_groups[i] {
                    if u < v {
                        ans += lca.dist(u, v);
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
