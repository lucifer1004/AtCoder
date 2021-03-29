use proconio::input;

fn main() {
    input! {
        t: usize,
        n: [usize; t],
    };

    println!(
        "{}",
        n.into_iter()
            .map(|x| (if x % 2 == 1 {
                "Odd"
            } else if x % 4 != 0 {
                "Same"
            } else {
                "Even"
            })
            .to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
