use proconio::input;
use proconio::marker::Usize1;

struct Context {
    ans: Vec<usize>,
    lis: Vec<usize>,
}

fn dfs(u: usize, p: usize, a: &Vec<usize>, adj: &Vec<Vec<usize>>, ctx: &mut Context) {
    ctx.ans[u] = ctx.lis.len();
    for v in adj[u].clone() {
        if v != p {
            let mut add = false;
            let mut tmp = 0;
            let mut lo = 0;
            let mut hi = ctx.lis.len() - 1;
            while lo <= hi {
                let mid = (lo + hi) >> 1;
                if ctx.lis[mid as usize] < a[v] {
                    lo = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    hi = mid - 1;
                }
            }
            if lo == ctx.lis.len() {
                add = true;
                ctx.lis.push(a[v]);
            } else {
                tmp = ctx.lis[lo];
                ctx.lis[lo] = a[v];
            }
            dfs(v, u, a, adj, ctx);
            if add {
                ctx.lis.pop();
            } else {
                ctx.lis[lo] = tmp;
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut ctx = Context {
        ans: vec![0; n],
        lis: vec![a[0]],
    };

    dfs(0, n, &a, &adj, &mut ctx);

    println!(
        "{}",
        ctx.ans
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
