use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [i32; n],
    }

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push(-p[i]);
        if heap.len() > k {
            heap.pop();
        }
        if heap.len() == k {
            println!("{}", -heap.peek().unwrap());
        }
    }
}
