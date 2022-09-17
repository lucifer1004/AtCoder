use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut ans = vec![];
    let mut now = x;
    while now != 0 {
        ans.push(now);
        now = (now - 1) & x;
    }
    ans.push(0);
    ans.reverse();
    for ai in ans {
        println!("{}", ai);
    }
}
