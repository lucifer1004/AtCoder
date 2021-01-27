use proconio::input;
use proconio::marker::Usize1;

struct Context {
    n: usize,
    hi: usize,
    adj: Vec<Vec<(usize, usize)>>,
    color: Vec<usize>,
}

fn dfs(ctx: &mut Context, u: usize, p: usize) {
    let mut c = 1;
    for (v, e) in ctx.adj[u].clone() {
        if p != ctx.n && c == ctx.color[p] {
            c += 1;
        }
        if e != p {
            ctx.color[e] = c;
            ctx.hi = ctx.hi.max(c);
            dfs(ctx, v, e);
            c += 1;
        }
    }
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut adj: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    for (i, (u, v)) in edges.into_iter().enumerate() {
        adj[u].push((v, i));
        adj[v].push((u, i));
    }

    let mut color = vec![0usize; n - 1];

    let mut ctx = Context {
        n,
        hi: 0,
        adj,
        color,
    };

    dfs(&mut ctx, 0, n);

    println!("{}", ctx.hi);
    for c in ctx.color {
        println!("{}", c);
    }
}
