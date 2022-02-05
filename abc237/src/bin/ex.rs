use proconio::input;
use std::collections::HashSet;

struct Hungary {
    pa: Vec<usize>,
    pb: Vec<usize>,
    vis: Vec<usize>,
    n: usize,
    dfn: usize,
    res: usize,
}

impl Hungary {
    fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n = graph.len() - 1;
        let vis = vec![0; n + 1];
        let pa = vec![0; n + 1];
        let pb = vec![0; n + 1];

        Self {
            pa,
            pb,
            vis,
            n,
            dfn: 0,
            res: 0,
        }
    }

    fn dfs(&mut self, u: usize, graph: &Vec<Vec<usize>>) -> bool {
        self.vis[u] = self.dfn;

        for &v in graph[u].iter() {
            if self.pb[v] == 0 {
                self.pa[u] = v;
                self.pb[v] = u;
                return true;
            }
        }

        for &v in graph[u].iter() {
            if self.vis[self.pb[v]] != self.dfn && self.dfs(self.pb[v], graph) {
                self.pa[u] = v;
                self.pb[v] = u;
                return true;
            }
        }

        false
    }

    fn solve(&mut self, graph: &Vec<Vec<usize>>) -> usize {
        loop {
            self.dfn += 1;
            let mut cnt = 0;
            for i in 1..=self.n {
                if self.pa[i] == 0 && self.dfs(i, graph) {
                    cnt += 1;
                }
            }

            if cnt == 0 {
                break;
            }

            self.res += cnt;
        }
        self.res
    }
}

fn main() {
    input! {
        s: String,
    }

    let chars = s.chars().collect::<Vec<_>>();
    let mut distinct_palindromes: HashSet<String> = HashSet::new();
    let n = s.len();
    for i in 0..n {
        for j in 0..=i.min(n - 1 - i) {
            if chars[i - j] != chars[i + j] {
                break;
            }

            let palindrome: String = s.chars().skip(i - j).take(2 * j + 1).collect();
            distinct_palindromes.insert(palindrome);
        }

        if i < n - 1 {
            for j in 0..=i.min(n - 2 - i) {
                if chars[i - j] != chars[i + 1 + j] {
                    break;
                }

                let palindrome: String = s.chars().skip(i - j).take(2 * j + 2).collect();
                distinct_palindromes.insert(palindrome);
            }
        }
    }

    let distinct_palindromes: Vec<String> = distinct_palindromes.into_iter().collect::<Vec<_>>();
    let n = distinct_palindromes.len();
    let mut graph = vec![vec![]; n + 1];
    for i in 0..n {
        for j in 0..n {
            if i != j && distinct_palindromes[j].contains(&distinct_palindromes[i]) {
                graph[i + 1].push(j + 1);
            }
        }
    }

    let mut hungary = Hungary::new(&graph);
    println!("{}", n - hungary.solve(&graph));
}
