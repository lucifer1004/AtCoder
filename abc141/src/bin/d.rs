use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        a: [usize; n],
    }
    let mut heap = BinaryHeap::new();
    let mut cost: usize = a.iter().sum();
    let mut tickets = vec![0usize; n];
    for i in 0usize..n {
        heap.push((a[i] - (a[i] >> 1), i));
    }
    while m > 0 && !heap.is_empty() {
        let (discount, idx) = heap.pop().unwrap();
        cost -= discount;
        m -= 1;
        tickets[idx] += 1;
        let next_discount = (a[idx] >> tickets[idx]) - (a[idx] >> (tickets[idx] + 1));
        if next_discount > 0 {
            heap.push((next_discount, idx));
        }
    }
    println!("{}", cost);
}
