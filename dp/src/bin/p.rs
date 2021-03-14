use proconio::input;
use proconio::marker::Usize1;

const MOD: usize = 1_000_000_007;

struct Context {
    adj: Vec<Vec<usize>>,
    parent: Vec<usize>,
    white: Vec<usize>,
    black: Vec<usize>,
}

fn dfs(u: usize, p: usize, ctx: &mut Context) {
    ctx.white[u] = 1;
    ctx.black[u] = 1;
    for v in ctx.adj[u].clone() {
        if v != p {
            ctx.parent[v] = u;
            dfs(v, u, ctx);
            ctx.white[u] = ctx.white[u] * (ctx.white[v] + ctx.black[v]) % MOD;
            ctx.black[u] = ctx.black[u] * ctx.white[v] % MOD;
        }
    }
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut ctx = Context {
        adj: vec![vec![]; n],
        parent: vec![n; n],
        white: vec![0usize; n],
        black: vec![0usize; n],
    };

    for (u, v) in edges {
        ctx.adj[u].push(v);
        ctx.adj[v].push(u);
    }

    dfs(0, n, &mut ctx);

    println!("{}", (ctx.white[0] + ctx.black[0]) % MOD);
}