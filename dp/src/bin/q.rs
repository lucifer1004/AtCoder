use proconio::input;

#[derive(Clone, Default)]
struct SegTreeNode {
    l: usize,
    r: usize,
    hi: usize,
}

struct SegTree {
    nodes: Vec<SegTreeNode>
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            nodes: vec![SegTreeNode::default(); (n << 2) + 10],
        }
    }

    fn calc(&mut self, idx: usize) {
        self.nodes[idx].hi = self.nodes[idx << 1].hi.max(self.nodes[idx << 1 | 1].hi);
    }

    fn build(&mut self, idx: usize, l: usize, r: usize) {
        self.nodes[idx].l = l;
        self.nodes[idx].r = r;
        if l < r {
            let mid = (l + r) >> 1;
            self.build(idx << 1, l, mid);
            self.build(idx << 1 | 1, mid + 1, r);
        }
    }

    fn update(&mut self, idx: usize, pos: usize, val: usize) {
        if self.nodes[idx].l == pos && self.nodes[idx].r == pos {
            self.nodes[idx].hi = self.nodes[idx].hi.max(val);
        } else {
            let mid = (self.nodes[idx].l + self.nodes[idx].r) >> 1;
            if pos <= mid {
                self.update(idx << 1, pos, val);
            } else {
                self.update(idx << 1 | 1, pos, val);
            }
            self.calc(idx);
        }
    }

    fn query(&self, idx: usize, l: usize, r: usize) -> usize {
        if self.nodes[idx].l >= l && self.nodes[idx].r <= r {
            self.nodes[idx].hi
        } else {
            let mut ans = 0;
            let mid = (self.nodes[idx].l + self.nodes[idx].r) >> 1;
            if l <= mid {
                ans = ans.max(self.query(idx << 1, l, r));
            }
            if mid + 1 <= r {
                ans = ans.max(self.query(idx << 1 | 1, l, r));
            }
            ans
        }
    }
}

fn main() {
    input! {
        n: usize,
        h: [usize; n],
        a: [usize; n],
    }

    let mut order = (0..n).collect::<Vec<usize>>();
    order.sort_by(|i, j| {
        if h[*i] < h[*j] {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut st = SegTree::new(n);
    st.build(1, 1, n);

    for i in order {
        let hi = st.query(1, 1, i + 1);
        st.update(1, i + 1, hi + a[i]);
    }

    println!("{}", st.query(1, 1, n));
}