use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let ans = s.iter().filter(|&x| x == "Takahashi").count();
    println!("{}", ans);
}
