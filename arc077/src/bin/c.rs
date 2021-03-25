use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = VecDeque::new();
    for (i, ai) in a.into_iter().enumerate() {
        if i % 2 == 0 {
            b.push_back(ai);
        } else {
            b.push_front(ai);
        }
    }

    if n % 2 == 0 {
        println!(
            "{}",
            b.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    } else {
        let mut b = b.into_iter().collect::<Vec<usize>>();
        b.reverse();
        println!(
            "{}",
            b.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
