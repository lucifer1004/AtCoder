use proconio::input;

fn dfs(c: u128, p: usize, x: u128, bags: &Vec<Vec<u128>>, ans: &mut usize) {
    if p == bags.len() {
        if c == x {
            *ans += 1;
        }
        return
    }

    for &num in bags[p].iter() {
        if c * num <= x && x % (c * num) == 0 {
            dfs(c * num, p + 1, x, bags, ans);
        }
    }
}

fn main() {
    input! {
        n: usize,
        x: u128,
        bags: [[u128]; n],
    }

    let mut ans = 0;
    dfs(1, 0, x, &bags, &mut ans);

    println!("{}", ans);
}
