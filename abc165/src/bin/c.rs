use proconio::input;

struct Context {
    best: usize,
}

fn dfs(
    now: &mut Vec<usize>,
    m: usize,
    n: usize,
    ctx: &mut Context,
    rules: &Vec<(usize, usize, usize, usize)>,
) {
    if now.len() == n + 1 {
        let mut score = 0;
        for i in 0..rules.len() {
            if now[rules[i].1] - now[rules[i].0] == rules[i].2 {
                score += rules[i].3;
            }
        }
        ctx.best = ctx.best.max(score);
    } else {
        let last = *now.last().unwrap();
        for i in last..=m {
            now.push(i);
            dfs(now, m, n, ctx, rules);
            now.pop();
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        rules: [(usize, usize, usize, usize); q],
    }

    let mut ctx = Context { best: 0 };
    let mut now = vec![1];
    dfs(&mut now, m, n, &mut ctx, &rules);

    println!("{}", ctx.best);
}
