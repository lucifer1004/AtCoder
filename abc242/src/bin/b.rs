use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut t = s.chars().collect::<Vec<_>>();
    t.sort();
    println!("{}", t.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""));
}
