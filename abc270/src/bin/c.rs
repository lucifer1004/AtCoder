use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut par = vec![n; n];

    // Do DFS from X
    let mut stack = vec![x];
    par[x] = x;
    while let Some(node) = stack.pop() {
        for &next in &graph[node] {
            if par[next] == n {
                par[next] = node;
                stack.push(next);
            }
        }
    }

    let mut path = vec![];
    let mut node = y;
    while node != x {
        path.push(node);
        node = par[node];
    }
    path.push(x);
    path.reverse();

    println!(
        "{}",
        path.iter()
            .map(|&x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
