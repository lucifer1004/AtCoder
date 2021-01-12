use proconio::input;

struct LinearBasis {
    a: Vec<u64>,
    len: usize,
}

impl LinearBasis {
    pub fn new(len: usize) -> Self {
        assert!(len > 0 && len <= 64);
        LinearBasis {
            a: vec![0u64; len],
            len,
        }
    }

    pub fn insert(&mut self, mut val: u64) {
        for k in (0..self.len).rev() {
            if val & (1u64 << k) != 0 {
                if self.a[k] > 0 {
                    val ^= self.a[k];
                } else {
                    for t in 0..k {
                        if val & (1u64 << t) > 0 {
                            val ^= self.a[t];
                        }
                    }
                    for t in k + 1..self.len {
                        if self.a[t] & (1u64 << k) > 0 {
                            self.a[t] ^= val;
                        }
                    }
                    self.a[k] = val;
                    return;
                }
            }
        }
    }

    pub fn max(&self) -> u64 {
        let mut ans = 0u64;
        for k in 0..60 {
            ans ^= self.a[k];
        }
        ans
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    let mut ans = 0u64;

    for k in 0..60 {
        let mask = 1u64 << k;
        let mut cnt = 0usize;
        for v in a.iter() {
            if v & mask > 0 {
                cnt += 1;
            }
        }
        if cnt % 2 == 1 {
            for v in a.iter_mut() {
                *v &= !mask;
            }
            ans += mask;
        }
    }

    let mut linear_basis = LinearBasis::new(60);

    for v in a.iter() {
        linear_basis.insert(*v);
    }

    ans += linear_basis.max() * 2;

    println!("{}", ans);
}
