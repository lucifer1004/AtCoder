use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};

use proconio::input;

const MOD: usize = 1_000_000_007;

#[derive(Clone, Copy)]
struct ModInt {
    val: usize,
}

impl ModInt {
    const TWO: ModInt = Self { val: 2 };
    const ONE: ModInt = Self { val: 1 };
    const ZERO: ModInt = Self { val: 0 };

    fn new(val: usize) -> Self {
        Self { val: val % MOD }
    }
}

impl Add for ModInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.val + other.val)
    }
}

impl AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        self.val = (self.val + other.val) % MOD;
    }
}

impl Sub for ModInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.val + MOD - other.val)
    }
}

impl SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        self.val = (self.val + MOD - other.val) % MOD;
    }
}

impl Mul for ModInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.val * other.val)
    }
}

impl MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        self.val = self.val * other.val % MOD;
    }
}

fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
    }

    c.sort();
    c.reverse();

    let mut two = ModInt::ONE;
    for i in 1..=n * 2 - 2 {
        two = two * ModInt::TWO;
    }

    let mut ans = ModInt::ZERO;
    for i in 0..n {
        ans += ModInt::new(i + 2) * ModInt::new(c[i]);
    }

    println!("{}", (ans * two).val);
}
