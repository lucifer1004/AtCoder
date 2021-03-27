use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut v = s.chars().collect::<Vec<char>>();
    let tmp = v[0];
    v[0] = v[1];
    v[1] = v[2];
    v[2] = tmp;

    println!(
        "{}",
        v.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
