// Basic modular arithmetic functions as described in
// https://codeforces.com/blog/entry/72527
// https://codeforces.com/blog/entry/72593

/// Coefficients of a linear congruential function f(x) = (a * x + b) mod m;
/// the value m is used to avoid a and b growing too big but there is still
/// risk of arithmetic overflow if m is too large
pub struct LCF {
    a: i128,
    b: i128,
    m: i128,
}

impl LCF {
    pub fn identity(m: i128) -> Self {
        Self { a: 1, b: 0, m }
    }

    pub fn new(a: i128, b: i128, m: i128) -> Self {
        Self { a, b, m }
    }

    /// Compose two linear congruential functions f(x) = (a * x + b) mod m
    /// and g(x) = (c * x + d) mod, returning h(x) = f(g(x)); the value m
    /// must be the same on both functions
    pub fn compose(&self, other: &Self) -> Option<Self> {
        if self.m != other.m {
            None
        } else {
            Some(Self {
                a: (self.a * other.a).rem_euclid(self.m),
                b: (self.b * other.a + other.b).rem_euclid(self.m),
                m: self.m,
            })
        }
    }

    /// Compose a linear congruential function f(x) = (a * x + b) mod m into
    /// itself k times; as the values can be very large, it needs to do
    /// exponentiation by squaring
    pub fn repeat(&self, k: u128) -> Self {
        let a = pow_mod(self.a, k, self.m);
        let b = ((self.b * (1 - a)).rem_euclid(self.m)
            * inv_mod(1 - self.a, self.m))
        .rem_euclid(self.m);
        Self { a, b, m: self.m }
    }

    /// Applies the function to argument x, i.e. returns the value
    /// f(x) = (a * x + b) mod m
    pub fn apply(&self, x: i128) -> i128 {
        (self.a * x + self.b).rem_euclid(self.m)
    }

    /// Applies the inverse of the function to argument v, i.e. returns the
    /// value x such that f(x) = (a * x + b) mod m = v; this requires an
    /// application of modular multiplicative inverse
    pub fn inverse(&self, v: i128) -> i128 {
        ((v - self.b) * inv_mod(self.a, self.m)).rem_euclid(self.m)
    }
}

// This function implements exponentiation by squaring to calculate the value
/// (a ^ k) mod m; it requires m to be prime
fn pow_mod(mut a: i128, mut k: u128, m: i128) -> i128 {
    // TODO: ensure m is prime
    if k == 0 {
        return 1;
    }
    let mut y = 1;
    while k > 1 {
        if k % 2 == 0 {
            k /= 2;
        } else {
            y = (a * y).rem_euclid(m);
            k = (k - 1) / 2;
        }
        a = (a * a).rem_euclid(m);
    }
    (a * y).rem_euclid(m)
}

/// Modular multiplicative inverse of a, i.e. it returns the value (a ^ -1)
/// such that a * (a ^ 1) mod m = 1; it only works for a mod m != 0
fn inv_mod(a: i128, m: i128) -> i128 {
    // TODO: ensure a mod m != 0 and m - 2 is non-negative
    pow_mod(a, (m - 2) as u128, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(0, 2, 5), 0);
        assert_eq!(pow_mod(2, 0, 5), 1);
        assert_eq!(pow_mod(2, 3, 5), 3);
        assert_eq!(pow_mod(3, 3, 5), 2);
        assert_eq!(pow_mod(3, 3, 7), 6);
        assert_eq!(pow_mod(3, 4, 11), 4);
        assert_eq!(pow_mod(4, 4, 13), 9);
        assert_eq!(pow_mod(4, 4, 17), 1);
        assert_eq!(pow_mod(5, 4, 17), 13);
    }
}
