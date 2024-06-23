use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut st = vec![];
    let mut dp = vec![0; n];
    dp[0] = h[0];
    st.push(0);

    for i in 1..n {
        while st.len() >= 1 && h[st[st.len() - 1]] < h[i] {
            st.pop();
        }
        dp[i] = if st.is_empty() {
            h[i] * (i + 1)
        } else {
            dp[st[st.len() - 1]] + h[i] * (i - st[st.len() - 1])
        };
        st.push(i);
    }

    for i in 0..n {
        print!("{} ", dp[i] + 1);
    }
}
