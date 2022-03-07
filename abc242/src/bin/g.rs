use proconio::input;
use std::cmp::Ordering;

const UNIT: usize = 100;

#[derive(Eq, PartialEq)]
struct Node {
    l: usize,
    r: usize,
    id: usize,
}

impl Node {
    fn new(l: usize, r: usize, id: usize) -> Self {
        Self { l, r, id }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.l / UNIT != other.l / UNIT {
            Some(self.l.cmp(&other.l))
        } else if (self.l / UNIT) & 1 == 1 { 
            Some(self.r.cmp(&other.r))
        } else {
            Some(other.r.cmp(&self.r))
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn update(x: usize, bits: &mut [bool], a: &[usize], bitcount: &mut usize) {
    bits[a[x - 1]] = !bits[a[x - 1]];
    if bits[a[x - 1]] {
        *bitcount += 1;
    } else {
        *bitcount -= 1;
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        queries: [(usize, usize); q],
    }

    let mut nodes = queries.iter().enumerate().map(|(i, q)| {
        Node::new(q.0, q.1, i)
    }).collect::<Vec<_>>();
    nodes.sort();

    let mut ans = vec![0usize; q];
    let mut l = 1;
    let mut r = 0;
    let mut bits = vec![false; n + 1];
    let mut bitcount = 0;

    for node in nodes {
        while l > node.l {
            l -= 1;
            update(l, &mut bits, &a, &mut bitcount);
        }

        while r < node.r {
            r += 1;
            update(r, &mut bits, &a, &mut bitcount);
        }

        while l < node.l {
            update(l, &mut bits, &a, &mut bitcount);
            l += 1;
        }

        while r > node.r {
            update(r, &mut bits, &a, &mut bitcount);
            r -= 1;
        }

        ans[node.id] = (node.r - node.l + 1 - bitcount) / 2;
    }

    for ansi in ans {
        println!("{}", ansi);
    }
}
