use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use proconio::input;
use proconio::marker::Usize1;

const MOD: usize = 1_000_000_007;

fn fexp(mut x: usize, mut y: usize) -> usize {
    let mut ans = 1;
    while y > 0 {
        if y & 1 == 1 {
            ans = ans * x % MOD;
        }
        x = x * x % MOD;
        y >>= 1;
    }
    ans
}

fn inv(x: usize) -> usize {
    fexp(x, MOD - 2)
}

#[derive(Clone, Copy)]
struct ModInt {
    val: usize,
}

impl ModInt {
    const ONE: ModInt = Self { val: 1 };
    const ZERO: ModInt = Self { val: 0 };

    fn new(val: usize) -> Self {
        Self { val: val % MOD }
    }
}

impl Add for ModInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.val + other.val)
    }
}

impl AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        self.val = (self.val + other.val) % MOD;
    }
}

impl Sub for ModInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.val + MOD - other.val)
    }
}

impl SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        self.val = (self.val + MOD - other.val) % MOD;
    }
}

impl Mul for ModInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.val * other.val)
    }
}

impl MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        self.val = self.val * other.val % MOD;
    }
}

impl Div for ModInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        assert_ne!(other.val, 0);
        Self::new(self.val * inv(other.val))
    }
}

impl DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        assert_ne!(other.val, 0);
        self.val = self.val * inv(other.val) % MOD;
    }
}

/// Store the context during DFS recursion.
///
/// `adj`: Adjacency list representation of the graph
/// `cnt`: Size of subtree
/// `exp`: Expectation of holes
/// `buffer_exp`: Expectation of white nodes that could become holes
/// `inv_two`: Precalculated inv of 2^k
struct Context {
    adj: Vec<Vec<usize>>,
    cnt: Vec<usize>,
    exp: Vec<ModInt>,
    buffer_exp: Vec<ModInt>,
    inv_two: Vec<ModInt>,
}

fn dfs(ctx: &mut Context, u: usize, p: usize) {
    let mut black = ModInt::ZERO;
    let mut white = ModInt::ZERO;
    let black_buffer = ModInt::ZERO;
    let mut white_buffer = ModInt::ZERO;
    for v in ctx.adj[u].clone() {
        if v != p {
            // Solve subtree, and accumulate node counters.
            dfs(ctx, v, u);
            ctx.cnt[u] += ctx.cnt[v];

            // If the current node is colored white, all holes remain.
            white += ctx.exp[v];

            // If the current node is colored black, all buffers also become holes.
            black += ctx.exp[v] + ctx.buffer_exp[v];
        }
    }

    let p_all_white = ctx.inv_two[ctx.cnt[u] - 1];
    let mut p_one_black = ModInt::ZERO; // Probability that exactly one subtree has black nodes.
    for v in ctx.adj[u].clone() {
        if v != p {
            let p_all_other_white = ctx.inv_two[ctx.cnt[u] - 1 - ctx.cnt[v]];
            let p_v_is_only_black = p_all_other_white * (ModInt::ONE - ctx.inv_two[ctx.cnt[v]]);
            p_one_black += p_v_is_only_black;

            // If all other subtrees are white, then the buffer of the subtree contributes to the buffer of the current node.
            white_buffer += p_all_other_white * ctx.buffer_exp[v];

            // If at least one other subtree has black nodes, then the buffer of the subtree contributes to holes.
            white += (ModInt::ONE - p_all_other_white) * ctx.buffer_exp[v];
        }
    }

    let p_at_least_two_black = ModInt::ONE - p_all_white - p_one_black;

    // If the current node is colored white, and at least two subtrees have black nodes, then the current node becomes a hole.
    white += p_at_least_two_black;

    // If the current node is colored white, and only one subtree has black nodes, then the current node contributes to the buffer.
    white_buffer += p_one_black;

    ctx.exp[u] = (black + white) * ctx.inv_two[1];
    ctx.buffer_exp[u] = (black_buffer + white_buffer) * ctx.inv_two[1];
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let half = ModInt::new(inv(2));
    let mut inv_two = vec![ModInt::new(1); n + 1];
    for i in 1..=n {
        inv_two[i] = inv_two[i - 1] * half;
    }

    let mut ctx = Context {
        adj,
        cnt: vec![1; n],
        exp: vec![ModInt::new(0); n],
        buffer_exp: vec![ModInt::new(0); n],
        inv_two,
    };

    dfs(&mut ctx, 0, n);

    println!("{}", ctx.exp[0].val);
}
