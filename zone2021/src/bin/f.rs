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
        m: usize,
        a: [usize; m],
    }

    if n == 2 {
        if m == 0 {
            println!("0 1");
        } else {
            println!("-1");
        }
        std::process::exit(0);
    }

    let mut banned = vec![false; n + 1];
    for i in a {
        banned[i] = true;
    }

    let mut dsu = DisjointSetUnion::new(n);
    let mut counter = 0;
    let mut ans = vec![];
    let mut chosen = vec![false; n + 1];

    for i in 1..n {
        if !banned[i] && !chosen[i] {
            for j in 0..n {
                if !dsu.is_connected(j, j ^ i) {
                    ans.push(format!("{} {}", j, j ^ i));
                    dsu.merge(j, j ^ i);
                    counter += 1;
                    if counter == n - 1 {
                        break;
                    }
                }
            }

            for j in 1..n {
                if chosen[j] {
                    chosen[j ^ i] = true;
                }
            }

            chosen[i] = true;
        }
    }

    if counter == n - 1 {
        println!("{}", ans.join("\n"));
    } else {
        println!("-1");
    }
}