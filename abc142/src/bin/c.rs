use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }

    let mut seq = a.into_iter().enumerate().collect::<Vec<(usize, i32)>>();

    seq.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());

    let ans = seq
        .into_iter()
        .map(|x| (x.0 + 1).to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", ans);
}
