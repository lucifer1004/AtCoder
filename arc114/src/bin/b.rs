use proconio::input;

const MOD: usize = 998_244_353;

struct Context {
    f: Vec<usize>,
    color: Vec<usize>,
    loops: usize,
}

fn dfs(u: usize, ctx: &mut Context) {
    ctx.color[u] = 1;
    let v = ctx.f[u] - 1;
    if ctx.color[v] == 0 {
        dfs(v, ctx);
    } else if ctx.color[v] == 1 {
        ctx.loops += 1;
    }
    ctx.color[u] = 2;
}

fn main() {
    input! {
        n: usize,
        f: [usize; n],
    }

    let mut ctx = Context {
        f,
        color: vec![0; n],
        loops: 0,
    };

    for i in 0..n {
        if ctx.color[i] == 0 {
            dfs(i, &mut ctx);
        }
    }

    let mut ans = 1;
    for _ in 0..ctx.loops {
        ans = ans * 2 % MOD;
    }
    ans -= 1;

    println!("{}", ans);
}