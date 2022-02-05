use proconio::input;

const N: usize = 200_005;

#[derive(Clone, Debug)]
struct SegmentTreeNode {
    l: usize,
    r: usize,
    sum: usize,
    lazy: bool,
}

impl SegmentTreeNode {
    fn new() -> Self {
        Self {
            l: 0,
            r: 0,
            sum: 0,
            lazy: false,
        }
    }

    fn range(&self) -> usize {
        self.r - self.l + 1
    }
}

#[derive(Clone, Debug)]
struct SegmentTree {
    nodes: Vec<SegmentTreeNode>,
}

impl SegmentTree {
    fn new() -> Self {
        Self {
            nodes: vec![SegmentTreeNode::new(); N << 2],
        }
    }

    fn calc(&mut self, idx: usize) {
        self.nodes[idx].sum = self.nodes[idx << 1].sum + self.nodes[idx << 1 | 1].sum;
    }

    fn push_down(&mut self, idx: usize) {
        if self.nodes[idx].lazy {
            if self.nodes[idx].sum == 0 {
                self.nodes[idx << 1].sum = 0;
                self.nodes[idx << 1].lazy = true;
                self.nodes[idx << 1 | 1].sum = 0;
                self.nodes[idx << 1 | 1].lazy = true;
            } else {
                self.nodes[idx << 1].sum = self.nodes[idx << 1].range();
                self.nodes[idx << 1].lazy = true;
                self.nodes[idx << 1 | 1].sum = self.nodes[idx << 1 | 1].range();
                self.nodes[idx << 1 | 1].lazy = true;
            }

            self.nodes[idx].lazy = false;
        }
    }

    fn build(&mut self, idx: usize, l: usize, r: usize, p: &Vec<usize>, x: usize) {
        self.nodes[idx].l = l;
        self.nodes[idx].r = r;
        if l == r {
            if p[l - 1] >= x {
                self.nodes[idx].sum = 1;
            }
        } else {
            let mid = (l + r) / 2;
            self.build(idx << 1, l, mid, p, x);
            self.build(idx << 1 | 1, mid + 1, r, p, x);
            self.calc(idx);
        }
    }

    fn modify(&mut self, idx: usize, l: usize, r: usize, val: usize) {
        if l > r {
            return
        }

        if self.nodes[idx].l >= l && self.nodes[idx].r <= r {
            if val == 0 {
                self.nodes[idx].sum = 0;
            } else {
                self.nodes[idx].sum = self.nodes[idx].range();
            }

            self.nodes[idx].lazy = true;
        } else {
            self.push_down(idx);
            let mid = (self.nodes[idx].l + self.nodes[idx].r) / 2;
            if mid >= l {
                self.modify(idx << 1, l, r, val);
            }
            if mid + 1 <= r {
                self.modify(idx << 1 | 1, l, r, val);
            }
            self.calc(idx);
        }
    }

    fn query(&mut self, idx: usize, l: usize, r: usize) -> usize {
        if self.nodes[idx].l >= l && self.nodes[idx].r <= r {
            self.nodes[idx].sum
        } else {
            self.push_down(idx);
            let mid = (self.nodes[idx].l + self.nodes[idx].r) / 2;
            let mut ans = 0;
            if mid >= l {
                ans += self.query(idx << 1, l, r);
            }
            if mid + 1 <= r {
                ans += self.query(idx << 1 | 1, l, r);
            }
            ans
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        x: usize,
        p: [usize; n],
        queries: [(usize, usize, usize); q],
    }

    let mut trees = vec![SegmentTree::new(); 2];
    trees[0].build(1, 1, n, &p, x);
    trees[1].build(1, 1, n, &p, x + 1);

    for (c, l, r) in queries {
        for i in 0..2 {
            let s = trees[i].query(1, l, r);
            if c == 1 {
                trees[i].modify(1, l, r - s, 0);
                trees[i].modify(1, r - s + 1, r, 1);
            } else {
                trees[i].modify(1, l, l + s - 1, 1);
                trees[i].modify(1, l + s, r, 0);
            }
        }
    }

    for i in 1..=n {
        if trees[0].query(1, i, i) != trees[1].query(1, i, i) {
            println!("{}", i);
            return;
        }
    }
}
