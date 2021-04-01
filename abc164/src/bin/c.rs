use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        items: [String; n],
    }

    println!("{}", items.into_iter().collect::<HashSet<String>>().len());
}
